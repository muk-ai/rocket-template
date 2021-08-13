#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use rocket::Build;
use rocket::Rocket;

mod config;
mod connection;
mod fairing;
mod firebase;
mod handlers;
mod id_token;
mod jwks;
mod log;
mod models;
mod schema;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();

    rocket::build()
        .manage(connection::init_pool())
        .attach(fairing::jwks::FetchJwksFairing)
        .attach(fairing::cors::CorsFairing)
        .attach(fairing::log::LoggingUidFairing)
        .attach(fairing::migration::MigrationFairing)
        .attach(handlers::auth::stage())
        .attach(handlers::tasks::stage())
        .attach(handlers::features_i_tried::stage())
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
