use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Mount /features-i-tried", |rocket| async {
        rocket.mount("/features-i-tried", routes![hello_world])
    })
}

#[get("/hello-world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}
