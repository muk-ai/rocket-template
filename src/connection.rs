use diesel::{r2d2::ConnectionManager, PgConnection};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::ops::Deref;

use crate::config::CONFIG;
use crate::log::{write_error, TraceContext};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(&CONFIG.database_url);
    PgPool::builder()
        .max_size(4)
        .build(manager)
        .expect("Failed to create pool")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<DbConn, Self::Error> {
        let trace = request.guard::<&TraceContext>().await.succeeded();

        if let Some(pool) = request.rocket().state::<PgPool>() {
            match pool.get() {
                Ok(conn) => Outcome::Success(DbConn(conn)),
                Err(_) => {
                    write_error("couldn't get connection from ConnectionManager", trace);
                    Outcome::Failure((Status::ServiceUnavailable, ()))
                }
            }
        } else {
            write_error("couldn't get PgPool", trace);
            Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
