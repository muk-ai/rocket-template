use once_cell::sync::OnceCell;
use serde::Deserialize;

pub const JWKS_URL: &str =
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
