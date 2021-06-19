use diesel::result::OptionalExtension;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome, Request};

use super::repository;
use super::User;
use crate::connection::DbConn;
use crate::firebase;
use crate::id_token::IdToken;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let conn = try_outcome!(request.guard::<DbConn>().await);
        let id_token = try_outcome!(request.guard::<IdToken>().await);

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
