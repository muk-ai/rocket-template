#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::fs::FileServer;
use rocket::Build;
use rocket::Rocket;

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
use handlers::cookies;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();

    rocket::build()
        .manage(connection::init_pool())
        .attach(fairing::jwks::FetchJwksFairing)
        .attach(fairing::cors::CorsFairing)
        .attach(fairing::log::LoggingUidFairing)
        .attach(fairing::migration::MigrationFairing)
        .attach(handlers::features_i_tried::stage())
        .mount("/", routes![cookies::cookies, cookies::set_cookies,])
        .mount("/public", FileServer::from(&CONFIG.public_dir))
        .attach(handlers::auth::stage())
        .attach(handlers::tasks::stage())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/features-i-tried/hello-world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
