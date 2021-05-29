use once_cell::sync::OnceCell;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use serde::Deserialize;

use crate::log::write_error;

const JWKS_URL: &str =
    "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
pub static FIREBASE_JWKS: OnceCell<JwkSet> = OnceCell::new();

#[derive(Debug, Deserialize)]
pub struct JwkSet {
    keys: Vec<Jwk>,
}

impl JwkSet {
    pub fn get_key(&self, kid: String) -> Option<&Jwk> {
        match self.keys.iter().find(|jwk| jwk.kid == kid) {
            Some(jwk) => Some(&jwk),
            None => None,
        }
    }
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
        match reqwest::blocking::get(JWKS_URL) {
            Ok(response) => match response.json::<JwkSet>() {
                Ok(json) => jwk_set = Some(json),
                Err(err) => {
                    write_error(format!("{:?}", err), None);
                }
            },
            Err(err) => write_error(format!("{:?}", err), None),
        }
        let jwk_set = jwk_set.expect(&format!("couldn't get JWK Set from {}", JWKS_URL));
        FIREBASE_JWKS
            .set(jwk_set)
            .expect("OnceCell<JwkSet> is already filled");
        Ok(rocket)
    }
}
