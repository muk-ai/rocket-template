use jsonwebtoken;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

pub struct IdToken(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub nbf: usize,
    pub sub: String,
    pub uid: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for IdToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let mut bearer_token: Option<String> = None;
        if let Some(authz_header) = request.headers().get_one("Authorization") {
            if authz_header.starts_with("Bearer ") {
                let token = authz_header[6..authz_header.len()].trim();
                bearer_token = Some(token.to_string());
            }
        }
        match bearer_token {
            Some(token) => Outcome::Success(IdToken(token)),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[get("/auth/me")]
pub fn get_auth_me(id_token: IdToken) -> String {
    let header = match jsonwebtoken::decode_header(&id_token.0) {
        Ok(header) => header,
        Err(_) => return "couldn't decode header".to_string(),
    };
    if header.alg != jsonwebtoken::Algorithm::RS256 {
        return "invalid algorithm".to_string();
    }
    id_token.0
}
