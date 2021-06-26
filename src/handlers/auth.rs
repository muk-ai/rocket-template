use diesel::result::OptionalExtension;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::connection::DbConn;
use crate::firebase;
use crate::id_token::IdToken;
use crate::models::users;
use crate::models::users::User;

#[get("/auth/me")]
pub fn get_auth_me(user: User) -> Json<User> {
    Json(user)
}

#[post("/auth/me")]
pub fn post_auth_me(id_token: IdToken, conn: DbConn) -> Result<Status, status::Custom<Value>> {
    let token_result = firebase::auth::verify_id_token(id_token.0);
    if let Err(message) = token_result {
        return Err(json_error(Status::Unauthorized, message));
    }

    let token_data = token_result.unwrap();
    let uid = token_data.claims.sub;
    match users::repository::find(uid.clone(), &conn).optional() {
        Ok(Some(_)) => Err(json_error(Status::Conflict, "conflict".to_string())),
        Ok(None) => match users::repository::insert(uid, &conn) {
            Ok(_) => Ok(Status::Created),
            Err(_) => Err(json_error(
                Status::InternalServerError,
                "internal server error".to_string(),
            )),
        },
        Err(_) => Err(json_error(
            Status::InternalServerError,
            "internal server error".to_string(),
        )),
    }
}

fn json_error(status: Status, message: String) -> status::Custom<Value> {
    let json_value = json!({
          "errors": [
              {
                  "detail": message
              }
          ]
      }
    );
    status::Custom(status, json_value)
}
