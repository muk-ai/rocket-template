use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::{Outcome, Request};
use rocket::Data;

use crate::log::{write_info, TraceContext};
use crate::models::users::User;

pub struct LoggingUidFairing;

#[rocket::async_trait]
impl Fairing for LoggingUidFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logging uid",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
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
