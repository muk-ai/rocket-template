use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use serde_json;
use serde_json::json;

use crate::config::CONFIG;

pub struct AccessLogFairing;

impl Fairing for AccessLogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Write Access Log",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, _response: &mut Response) {
        if let Some(header) = request.headers().get_one("X-Cloud-Trace-Context") {
            let chunks: Vec<&str> = header.split(&['/', ';'][..]).collect();
            if let (Some(trace), Some(span)) = (chunks.get(0), chunks.get(1)) {
                let trace = format!("projects/{}/traces/{}", &CONFIG.gcp_project_id, trace);
                let log = json! {
                    {
                        "severity": "INFO",
                        "message": format!("hello, cloud logging. header is {}", header),
                        "logging.googleapis.com/trace": trace,
                        "logging.googleapis.com/spanId": span
                    }
                };

                if let Ok(log) = serde_json::to_string(&log) {
                    println!("{}", log);
                }
            }
        }
    }
}
