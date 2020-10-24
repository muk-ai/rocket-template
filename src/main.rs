#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/count")]
fn count() -> &'static str {
  "couunt"
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, count])
    .launch();
}
