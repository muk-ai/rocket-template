use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::ops::Deref;

use crate::config::CONFIG;

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
        if let Some(pool) = request.rocket().state::<PgPool>() {
            match pool.get() {
                Ok(conn) => Outcome::Success(DbConn(conn)),
                Err(_) => {
                    eprintln!("couldn't get connection from ConnectionManager");
                    Outcome::Failure((Status::ServiceUnavailable, ()))
                }
            }
        } else {
            eprintln!("couldn't get PgPool");
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
