use crate::config::CONFIG;
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Mount /features-i-tried", |rocket| async {
        rocket
            .mount(
                "/features-i-tried",
                routes![hello_world, params, count, set_cookies, cookies],
            )
            .mount("/public", FileServer::from(&CONFIG.public_dir))
    })
}

#[get("/hello-world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/params/<id>")]
fn params(id: Option<usize>) -> String {
    match id {
        Some(n) => format!("usize: {n}"),
        None => "Not a usize".to_string(),
    }
}

use rocket::request::{FromRequest, Outcome, Request};
use std::sync::atomic::{AtomicUsize, Ordering};
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
struct RequestId(pub usize);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r RequestId {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.
        Outcome::Success(
            request.local_cache(|| RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
        )
    }
}
#[get("/count")]
fn count(id: &RequestId) -> String {
    format!("This is request #{}.", id.0)
}

use rocket::http::{Cookie, CookieJar, SameSite};
use time::{Duration, OffsetDateTime};
#[get("/set_cookies")]
fn set_cookies(cookies: &CookieJar<'_>) -> &'static str {
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
fn cookies(cookies: &CookieJar<'_>) -> Option<String> {
    let message = cookies.get("message").map(|crumb| crumb.value().to_owned());
    let private = cookies
        .get_private("private")
        .map(|crumb| crumb.value().to_owned());
    if let (Some(message), Some(private)) = (message, private) {
        Some(format!("message: {message}, private: {private}"))
    } else {
        None
    }
}
