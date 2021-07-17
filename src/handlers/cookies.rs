use rocket::http::{Cookie, CookieJar, SameSite};
use time::{Duration, OffsetDateTime};

#[get("/cookies")]
pub fn cookies(cookies: &CookieJar<'_>) -> Option<String> {
    let one_month_later = OffsetDateTime::now_utc() + Duration::weeks(4);
    let cookie = Cookie::build("first", "value")
        .http_only(true)
        .same_site(SameSite::Lax)
        .expires(one_month_later)
        .finish();
    cookies.add(cookie);
    cookies
        .get("message")
        .map(|crumb| format!("Message: {}", crumb.value()))
}
