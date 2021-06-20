use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub struct HitCount(AtomicUsize);

impl HitCount {
    pub fn new() -> Self {
        HitCount(AtomicUsize::new(0))
    }
}

#[get("/count")]
pub fn count(hit_count: &State<HitCount>) -> String {
    let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
    format!("This is request #{}.", count)
}
