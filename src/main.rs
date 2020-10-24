#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::request::{self, Request, FromRequest};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
struct RequestId(pub usize);

impl<'a, 'r> FromRequest<'a, 'r> for &'a RequestId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.
        request::Outcome::Success(request.local_cache(|| {
            RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
        }))
    }
}

#[get("/count")]
fn count(id: &RequestId) -> String {
    format!("This is request #{}.", id.0)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, count])
    .launch();
}
