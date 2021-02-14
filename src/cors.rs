use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{http::Method, http::Status, Request, Response};
use std::io::Cursor;

use crate::config::CONFIG;

pub struct CorsFairing;

impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            &CONFIG.allowed_origin,
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "DELETE, GET, HEAD, OPTIONS, PATCH, POST, PUT",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        // NOTE: replace status code and body if not found
        if response.status() == Status::NotFound && request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
