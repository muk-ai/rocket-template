use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Rocket};

use crate::jwks::{JwkSet, FIREBASE_JWKS, JWKS_URL};
use crate::log::write_error;

pub struct FetchJwksFairing;

#[rocket::async_trait]
impl Fairing for FetchJwksFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fetch JWK Set",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let mut jwk_set: Option<JwkSet> = None;
        match reqwest::get(JWKS_URL).await {
            Ok(response) => match response.json::<JwkSet>().await {
                Ok(json) => jwk_set = Some(json),
                Err(err) => {
                    write_error(format!("{err:?}"), None);
                }
            },
            Err(err) => write_error(format!("{err:?}"), None),
        }
        let jwk_set = jwk_set.unwrap_or_else(|| panic!("couldn't get JWK Set from {JWKS_URL}"));
        FIREBASE_JWKS
            .set(jwk_set)
            .expect("OnceCell<JwkSet> is already filled");
        Ok(rocket)
    }
}
