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

mod connection;

mod handlers;
use handlers::count;
use handlers::hello_world;
use handlers::params;

use rocket::State;

#[get("/tasks/<id>")]
fn tasks_get(
    id: i32,
    pool: State<connection::PgPool>,
) -> Result<Json<Task>, diesel::result::Error> {
    use diesel::prelude::*;

    let conn = pool.get().unwrap();
    let query_result: QueryResult<Task> = tasks::table.find(id).get_result::<Task>(&conn);
    query_result.map(|task| Json(task))
}

fn main() {
    use dotenv::dotenv;
    dotenv().ok();

    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![hello_world::index])
        .mount("/", routes![count::count, params::params, tasks_get])
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
        .launch();
}
