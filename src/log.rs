use std::net::SocketAddr;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

pub struct AccessLogFairing;

impl Fairing for AccessLogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Write Access Log",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if let Some(header) = request.headers().get_one("X-Cloud-Trace-Context") {
            let chunks: Vec<&str> = header.split(&['/', ';'][..]).collect();
            if let (Some(trace), Some(span)) = (chunks.get(0), chunks.get(1)) {
                println!("{}", header);
                println!("{}", trace);
                println!("{}", span);
            }
        }

        println!("{}", request.method().as_str());
        println!("{}", response.status().code);
        println!("{}", request.uri());
        println!("{}", request.uri().path());
        println!("{}", remote_as_string(request.remote()));
    }
}

fn remote_as_string(remote: Option<SocketAddr>) -> String {
    match remote {
        Some(addr) => addr.to_string(),
        None => '-'.to_string(),
    }
}
