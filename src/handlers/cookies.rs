use rocket::http::{Cookie, CookieJar};

#[get("/cookies")]
pub fn cookies(cookies: &CookieJar<'_>) -> Option<String> {
    cookies.add(Cookie::new("first", "value"));
    cookies
        .get("message")
        .map(|crumb| format!("Message: {}", crumb.value()))
}
