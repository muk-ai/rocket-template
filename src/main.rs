#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_contrib::serve::StaticFiles;

mod connection;
mod schema;
mod task;

mod handlers;
use handlers::count;
use handlers::hello_world;
use handlers::params;
use handlers::tasks;

fn main() {
    use dotenv::dotenv;
    dotenv().ok();

    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/", routes![hello_world::index])
        .mount("/", routes![count::count, params::params, tasks::tasks_get])
        .mount(
            "/public",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
        .launch();
}
