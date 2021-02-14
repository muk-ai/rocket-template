use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub allowed_origin: String,
}

impl Config {
    fn from_env() -> Config {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("environment variable DATABASE_URL is not defined");
        let allowed_origin =
            env::var("ALLOWED_ORIGIN").expect("environment variable ALLOWED_ORIGIN is not defined");
        Config {
            database_url,
            allowed_origin,
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_env());
