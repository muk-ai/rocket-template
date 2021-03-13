use crate::models::users::User;

#[get("/auth/me")]
pub fn get_auth_me(user: User) -> String {
    format!("{:?}", user)
}
