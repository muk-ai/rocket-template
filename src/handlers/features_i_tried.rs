use rocket::fairing::AdHoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Mount /features-i-tried", |rocket| async {
        rocket.mount("/features-i-tried", routes![hello_world, params])
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
