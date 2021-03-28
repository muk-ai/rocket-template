use diesel::result::OptionalExtension;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use super::repository;
use super::User;
use crate::connection::DbConn;
use crate::firebase;
use crate::id_token::IdToken;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn = request.guard::<DbConn>()?;
        let id_token = request.guard::<IdToken>()?;

        match firebase::auth::verify_id_token(id_token.0) {
            Ok(token_data) => {
                let uid = token_data.claims.sub;
                match repository::find(uid, &conn).optional() {
                    Ok(Some(user)) => Outcome::Success(user),
                    Ok(None) => Outcome::Failure((Status::NotFound, ())),
                    Err(_) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
