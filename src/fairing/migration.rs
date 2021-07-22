use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Rocket};

use crate::connection::PgPool;
use crate::log::write_error;

embed_migrations!();

pub struct MigrationFairing;

#[rocket::async_trait]
impl Fairing for MigrationFairing {
    fn info(&self) -> Info {
        Info {
            name: "Execute DB Migration",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let pool = rocket
            .state::<PgPool>()
            .expect("couldn't get connection pool from state");
        if run_db_migrations(pool).is_ok() {
            Ok(rocket)
        } else {
            Err(rocket)
        }
    }
}

fn run_db_migrations(pool: &PgPool) -> Result<(), ()> {
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
