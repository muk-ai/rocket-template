use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;

use crate::jwks::{JwkSet, FIREBASE_JWKS, JWKS_URL};
use crate::log::write_error;

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
