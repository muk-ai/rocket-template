use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2;
use std::env;

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
