use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::Serialize;
use serde_json;
use serde_json::json;

use crate::config::CONFIG;

pub struct TraceContext {
    trace: String,
    span_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r TraceContext {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
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

#[derive(Serialize)]
enum LogSeverity {
    INFO,
    ERROR,
}

fn write_log(severity: LogSeverity, message: impl Into<String>, context: Option<&TraceContext>) {
    let message: String = message.into();
    let log = match context {
        Some(context) => {
            json! {
                {
                    "severity": severity,
                    "message": message,
                    "logging.googleapis.com/trace": context.trace,
                    "logging.googleapis.com/spanId": context.span_id
                }
            }
        }
        None => {
            json! {
                {
                    "severity": severity,
                    "message": message,
                }
            }
        }
    };
    if let Ok(log) = serde_json::to_string(&log) {
        match severity {
            LogSeverity::INFO => println!("{}", log),
            LogSeverity::ERROR => eprintln!("{}", log),
        }
    }
}

pub fn write_info(message: impl Into<String>, context: Option<&TraceContext>) {
    write_log(LogSeverity::INFO, message, context);
}

pub fn write_error(message: impl Into<String>, context: Option<&TraceContext>) {
    write_log(LogSeverity::ERROR, message, context);
}
