use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::Response;
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

pub struct TraceContext {
    trace: String,
    span_id: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for &'a TraceContext {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let trace_context = request.local_cache(|| {
            if let Some(header) = request.headers().get_one("X-Cloud-Trace-Context") {
                let chunks: Vec<&str> = header.split(&['/', ';'][..]).collect();
                if let (Some(trace), Some(span)) = (chunks.get(0), chunks.get(1)) {
                    let trace = format!("projects/{}/traces/{}", &CONFIG.gcp_project_id, trace);
                    return Some(TraceContext {
                        trace: trace,
                        span_id: span.to_string(),
                    });
                }
            }
            return None;
        });
        match trace_context {
            Some(trace) => Outcome::Success(trace),
            None => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}

pub fn write_log(message: impl Into<String>, context: Option<&TraceContext>) {
    let message: String = message.into();
    let log = match context {
        Some(context) => {
            json! {
                {
                    "severity": "INFO",
                    "message": message,
                    "logging.googleapis.com/trace": context.trace,
                    "logging.googleapis.com/spanId": context.span_id
                }
            }
        }
        None => {
            json! {
                {
                    "severity": "INFO",
                    "message": message,
                }
            }
        }
    };
    if let Ok(log) = serde_json::to_string(&log) {
        println!("{}", log);
    }
}
