#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket_contrib::serve::StaticFiles;

mod config;
use config::CONFIG;

mod cors;
mod db;
mod jwks;
mod log;

mod connection;

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

fn main() {
    rocket::ignite()
        .manage(connection::init_pool())
        .attach(jwks::FetchJwksFairing)
        .attach(cors::CorsFairing)
        .attach(log::LoggingUidFairing)
        .attach(db::MigrationFairing)
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
