#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fs::FileServer;

mod config;
use config::CONFIG;

mod fairing;

mod connection;
mod firebase;
mod id_token;
mod jwks;
mod log;
mod models;
mod schema;

mod handlers;
use handlers::auth;
use handlers::cookies;
use handlers::count;
use handlers::hello_world;
use handlers::params;
use handlers::tasks;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .manage(connection::init_pool())
        .attach(fairing::jwks::FetchJwksFairing)
        .attach(fairing::cors::CorsFairing)
        .attach(fairing::log::LoggingUidFairing)
        .attach(fairing::migration::MigrationFairing)
        .mount("/", routes![hello_world::index])
        .mount(
            "/",
            routes![
                count::count,
                params::params,
                cookies::cookies,
                cookies::set_cookies,
                tasks::tasks_index,
                tasks::tasks_get,
                tasks::tasks_post,
                tasks::tasks_update,
                tasks::tasks_delete
            ],
        )
        .mount("/", routes![auth::get_auth_me, auth::post_auth_me])
        .mount("/public", FileServer::from(&CONFIG.public_dir))
        .ignite()
        .await?
        .launch()
        .await
}
