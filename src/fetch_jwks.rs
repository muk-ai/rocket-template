use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

const JWKS_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";

pub struct FetchJwksFairing;

impl Fairing for FetchJwksFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fetch JWK Set",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        if let Ok(response) = reqwest::blocking::get(JWKS_URL) {
            println!("{:?}", response)
        }
        Ok(rocket)
    }
}
