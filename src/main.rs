#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::{json::Json, serve::StaticFiles};

#[macro_use]
extern crate diesel;

mod task;
use task::Task;

mod schema;
use schema::tasks;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use rocket::{
    request::{self, FromRequest, Request},
    State,
};
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
struct RequestId(pub usize);

impl<'a, 'r> FromRequest<'a, 'r> for &'a RequestId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.
        request::Outcome::Success(
            request.local_cache(|| RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
        )
    }
}

#[get("/count")]
fn count(id: &RequestId) -> String {
    format!("This is request #{}.", id.0)
}

#[get("/params/<id>")]
fn params(id: Option<usize>) -> String {
    match id {
        Some(n) => format!("usize: {}", n),
        None => "Not a usize".to_string(),
    }
}

#[get("/tasks/<id>")]
fn tasks_get(id: i32, pool: State<PgPool>) -> Result<Json<Task>, diesel::result::Error> {
    use diesel::prelude::*;

    let conn = pool.get().unwrap();
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&conn);
    query_result.map(|task| Json(task))
}

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2;
use std::env;
type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    PgPool::builder()
        .max_size(4)
        .build(manager)
        .expect("Failed to create pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn main() {
    use dotenv::dotenv;
    dotenv().ok();

    rocket::ignite()
        .manage(init_pool())
        .mount("/", routes![index, count, params, tasks_get])
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
        .launch();
}
