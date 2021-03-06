use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Rocket};

use crate::connection::PgPool;
use crate::log::write_error;

embed_migrations!();

pub struct MigrationFairing;

#[rocket::async_trait]
impl Fairing for MigrationFairing {
    fn info(&self) -> Info {
        Info {
            name: "Execute DB Migration",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        if run_db_migrations(rocket).is_err() {
            panic!("migration failed, panic!")
        }
    }
}

fn run_db_migrations(rocket: &Rocket<Orbit>) -> Result<(), ()> {
    let pool = rocket
        .state::<PgPool>()
        .expect("couldn't get connection pool from state");
    match pool.get() {
        Ok(conn) => match embedded_migrations::run(&*conn) {
            Ok(()) => Ok(()),
            Err(e) => {
                write_error("migration failed", None);
                write_error(format!("Error: {}", e), None);
                Err(())
            }
        },
        Err(e) => {
            write_error("couldn't get connection pool", None);
            write_error(format!("Error: {}", e), None);
            Err(())
        }
    }
}
