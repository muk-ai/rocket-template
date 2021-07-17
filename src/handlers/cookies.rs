use rocket::http::CookieJar;

#[get("/cookies")]
pub fn cookies(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get("message")
        .map(|crumb| format!("Message: {}", crumb.value()))
}
