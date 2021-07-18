use rocket::http::{Cookie, CookieJar, SameSite};
use time::{Duration, OffsetDateTime};

#[get("/set_cookies")]
pub fn set_cookies(cookies: &CookieJar<'_>) -> &'static str {
    let one_month_later = OffsetDateTime::now_utc() + Duration::weeks(4);
    let cookie1 = Cookie::build("message", "value")
        .http_only(true)
        .same_site(SameSite::Lax)
        .expires(one_month_later)
        .finish();
    cookies.add(cookie1);
    let cookie2 = Cookie::build("private", "private value")
        .http_only(true)
        .same_site(SameSite::Lax)
        .expires(one_month_later)
        .finish();
    cookies.add_private(cookie2);
    "ok"
}

#[get("/cookies")]
pub fn cookies(cookies: &CookieJar<'_>) -> Option<String> {
    let message = cookies.get("message").map(|crumb| crumb.value().to_owned());
    let private = cookies
        .get_private("private")
        .map(|crumb| crumb.value().to_owned());
    if let (Some(message), Some(private)) = (message, private) {
        Some(format!("message: {}, private: {}", message, private))
    } else {
        None
    }
}
