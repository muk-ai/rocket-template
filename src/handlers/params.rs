#[get("/params/<id>")]
pub fn params(id: Option<usize>) -> String {
    match id {
        Some(n) => format!("usize: {}", n),
        None => "Not a usize".to_string(),
    }
}
