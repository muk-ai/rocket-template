use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::Data;
use serde_json;
use serde_json::json;

use crate::config::CONFIG;
use crate::models::users::User;

pub struct LoggingUidFairing;

impl Fairing for LoggingUidFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logging uid",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        let trace_context = match request.guard::<&TraceContext>() {
            Outcome::Success(context) => Some(context),
            _ => None,
        };
        let user = request.guard::<User>();
        match user {
            Outcome::Success(user) => {
                write_info(format!("firebase uid: {}", user.uid), trace_context);
                write_info(format!("users id: {}", user.id), trace_context);
            }
            _ => {
                write_info("user is anonymous", trace_context);
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

pub fn write_info(message: impl Into<String>, context: Option<&TraceContext>) {
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

pub fn write_error(message: impl Into<String>, context: Option<&TraceContext>) {
    let message: String = message.into();
    let log = match context {
        Some(context) => {
            json! {
                {
                    "severity": "ERROR",
                    "message": message,
                    "logging.googleapis.com/trace": context.trace,
                    "logging.googleapis.com/spanId": context.span_id
                }
            }
        }
        None => {
            json! {
                {
                    "severity": "ERROR",
                    "message": message,
                }
            }
        }
    };
    if let Ok(log) = serde_json::to_string(&log) {
        eprintln!("{}", log);
    }
}
