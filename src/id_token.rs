use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct IdToken(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for IdToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let mut bearer_token: Option<String> = None;
        if let Some(authz_header) = request.headers().get_one("Authorization") {
            if authz_header.starts_with("Bearer ") {
                bearer_token = authz_header
                    .split_once(' ')
                    .map(|(_, token)| token.trim().to_string());
            }
        }
        match bearer_token {
            Some(token) => Outcome::Success(IdToken(token)),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
