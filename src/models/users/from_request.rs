use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use super::User;
use crate::firebase;
use crate::id_token::IdToken;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let id_token = request.guard::<IdToken>()?;

        match firebase::auth::verify_id_token(id_token.0) {
            Ok(token_data) => {
                let uid = token_data.claims.sub;
                Outcome::Success(User { uid })
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
