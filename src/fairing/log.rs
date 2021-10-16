use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::{Outcome, Request};
use rocket::Data;

use crate::log::{write_info, TraceContext};
use crate::models::users::User;

pub struct LoggingFirebaseUidFairing;

#[rocket::async_trait]
impl Fairing for LoggingFirebaseUidFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logging firebase_uid",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let trace_context = match request.guard::<&TraceContext>().await {
            Outcome::Success(context) => Some(context),
            _ => None,
        };
        let user = request.guard::<User>().await;
        match user {
            Outcome::Success(user) => {
                write_info(
                    format!("users.firebase_uid is {}", user.firebase_uid),
                    trace_context,
                );
                write_info(format!("users.id is {}", user.id), trace_context);
            }
            _ => {
                write_info("user is anonymous", trace_context);
            }
        }
    }
}
