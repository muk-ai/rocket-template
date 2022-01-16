use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;
use crate::jwks::FIREBASE_JWKS;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub sub: String,
}

pub fn verify_id_token(id_token: String) -> Result<TokenData<Claims>, String> {
    let header = match decode_header(&id_token) {
        Ok(header) => header,
        Err(_) => return Err(String::from("couldn't decode header")),
    };
    let kid = match header.kid {
        Some(kid) => kid,
        None => return Err(String::from("kid is not found")),
    };
    let jwks = FIREBASE_JWKS.get().unwrap();
    let jwk = match jwks.get_key(kid) {
        Some(jwk) => jwk,
        None => return Err(String::from("JWK is not found")),
    };
    let project_id = &CONFIG.firebase_project_id;
    let mut validation = Validation {
        validate_exp: true,
        iss: Some("https://securetoken.google.com/".to_string() + project_id),
        ..Validation::new(Algorithm::RS256)
    };
    validation.set_audience(&[project_id]);
    let decoding_key = DecodingKey::from_rsa_components(&jwk.n, &jwk.e);
    let decoded_token = decode::<Claims>(&id_token, &decoding_key, &validation);
    let token_data = match decoded_token {
        Ok(token) => token,
        Err(e) => return Err(format!("{e}")),
    };
    Ok(token_data)
}
