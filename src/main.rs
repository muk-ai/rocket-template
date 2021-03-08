#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::{fairing::AdHoc, Rocket};
use rocket_contrib::serve::StaticFiles;

mod config;
use config::CONFIG;

mod cors;
mod jwks;

mod connection;
use connection::PgPool;

mod schema;
mod task;

mod firebase;
mod id_token;

mod handlers;
use handlers::auth;
use handlers::count;
use handlers::hello_world;
use handlers::params;
use handlers::tasks;

embed_migrations!();

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let pool = rocket
        .state::<PgPool>()
        .expect("could't get connection pool.");
    match pool.get() {
        Ok(conn) => match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(rocket),
            Err(_e) => Err(rocket),
        },
        Err(_e) => Err(rocket),
    }
}

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(jwks::FetchJwksFairing)
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(cors::CorsFairing)
        .mount("/", routes![hello_world::index])
        .mount(
            "/",
            routes![
                count::count,
                params::params,
                tasks::tasks_index,
                tasks::tasks_get,
                tasks::tasks_post,
                tasks::tasks_update,
                tasks::tasks_delete
            ],
        )
        .mount("/", routes![auth::get_auth_me])
        .mount("/public", StaticFiles::from(&CONFIG.public_dir))
        .launch();
}
