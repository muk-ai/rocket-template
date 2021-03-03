use rocket::http::Status;

#[get("/auth/me")]
pub fn get_auth_me() -> Status {
    Status::Ok
}
