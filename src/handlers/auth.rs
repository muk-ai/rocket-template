use diesel::result::OptionalExtension;
use rocket::http::Status;

use crate::connection::DbConn;
use crate::firebase;
use crate::id_token::IdToken;
use crate::models::users;
use crate::models::users::User;

#[get("/auth/me")]
pub fn get_auth_me(user: User) -> String {
    format!("{:?}", user)
}

#[post("/auth/me")]
pub fn post_auth_me(id_token: IdToken, conn: DbConn) -> Result<Status, Status> {
    firebase::auth::verify_id_token(id_token.0)
        .or(Err(Status::Unauthorized))
        .map(|token_data| token_data.claims.sub)
        .and_then(
            |uid: String| match users::repository::find(uid.clone(), &conn).optional() {
                Ok(Some(_)) => Err(Status::Conflict),
                Ok(None) => match users::repository::insert(uid, &conn) {
                    Ok(_) => Ok(Status::Created),
                    Err(_) => Err(Status::InternalServerError),
                },
                Err(_) => Err(Status::InternalServerError),
            },
        )
}
