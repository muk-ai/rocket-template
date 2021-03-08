use crate::firebase;
use crate::id_token::IdToken;

#[get("/auth/me")]
pub fn get_auth_me(id_token: IdToken) -> String {
    firebase::auth::verify_id_token(id_token.0)
}
