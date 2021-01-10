use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::env;
use std::ops::Deref;

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    PgPool::builder()
        .max_size(4)
        .build(manager)
        .expect("Failed to create pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
