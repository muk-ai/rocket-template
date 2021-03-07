use once_cell::sync::OnceCell;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use serde::Deserialize;

const JWKS_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
pub static FIREBASE_JWKS: OnceCell<JwkSet> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct JwkSet {
    keys: Vec<Jwk>,
}

#[derive(Debug, Deserialize)]
pub struct Jwk {
    pub e: String,
    pub alg: String,
    pub kty: String,
    pub kid: String,
    pub n: String,
}

pub struct FetchJwksFairing;

impl Fairing for FetchJwksFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fetch JWK Set",
            kind: Kind::Attach,
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        let mut jwk_set: Option<JwkSet> = None;
        if let Ok(response) = reqwest::blocking::get(JWKS_URL) {
            if let Ok(json) = response.json::<JwkSet>() {
                jwk_set = Some(json);
            }
        }
        let jwk_set = jwk_set.expect(&format!("couldn't get JWK Set from {}", JWKS_URL));
        FIREBASE_JWKS
            .set(jwk_set)
            .expect("OnceCell<JwkSet> is already filled");
        Ok(rocket)
    }
}
