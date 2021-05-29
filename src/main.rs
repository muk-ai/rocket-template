#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
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
mod log;

mod connection;
use connection::PgPool;

mod models;
mod schema;

mod firebase;
mod id_token;

mod handlers;
use handlers::auth;
use handlers::count;
use handlers::hello_world;
use handlers::params;
use handlers::tasks;

use log::write_error;

embed_migrations!();

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let pool = rocket
        .state::<PgPool>()
        .expect("couldn't get connection pool from state");
    match pool.get() {
        Ok(conn) => match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                write_error("migration failed", None);
                write_error(format!("Error: {}", e), None);
                Err(rocket)
            }
        },
        Err(e) => {
            write_error("couldn't get connection pool", None);
            write_error(format!("Error: {}", e), None);
            Err(rocket)
        }
    }
}

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(jwks::FetchJwksFairing)
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(cors::CorsFairing)
        .attach(log::LoggingUidFairing)
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
        .mount("/", routes![auth::get_auth_me, auth::post_auth_me])
        .mount("/public", StaticFiles::from(&CONFIG.public_dir))
        .launch();
}
