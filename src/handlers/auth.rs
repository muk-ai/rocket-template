use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct IdToken(String);

impl<'a, 'r> FromRequest<'a, 'r> for IdToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");
        match token {
            Some(token) => Outcome::Success(IdToken(token.to_string())),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[get("/auth/me")]
pub fn get_auth_me(id_token: IdToken) -> String {
    id_token.0
}
