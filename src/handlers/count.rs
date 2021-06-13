use rocket::request::{FromRequest, Outcome, Request};
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct RequestId(pub usize);

impl<'r> FromRequest<'r> for &RequestId {
    type Error = ();

    fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.
        Outcome::Success(
            request.local_cache(|| RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
        )
    }
}

#[get("/count")]
pub fn count(id: &RequestId) -> String {
    format!("This is request #{}.", id.0)
}
