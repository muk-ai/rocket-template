use rocket::http::{Cookie, CookieJar, SameSite};
use time::{Duration, OffsetDateTime};

#[get("/set_cookies")]
pub fn set_cookies(cookies: &CookieJar<'_>) -> &'static str {
    let one_month_later = OffsetDateTime::now_utc() + Duration::weeks(4);
    let cookie = Cookie::build("message", "value")
        .http_only(true)
        .same_site(SameSite::Lax)
        .expires(one_month_later)
        .finish();
    cookies.add(cookie);
    "ok"
}

#[get("/cookies")]
pub fn cookies(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get("message")
        .map(|crumb| format!("Message: {}", crumb.value()))
}
