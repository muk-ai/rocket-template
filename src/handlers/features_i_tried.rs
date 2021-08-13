use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Mount /features-i-tried", |rocket| async {
        rocket.mount("/features-i-tried", routes![hello_world, params, count])
    })
}

#[get("/hello-world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[get("/params/<id>")]
fn params(id: Option<usize>) -> String {
    match id {
        Some(n) => format!("usize: {}", n),
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
