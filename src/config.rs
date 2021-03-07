use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub allowed_origin: String,
    pub public_dir: String,
    pub firebase_project_id: String,
}

impl Config {
    fn from_env() -> Config {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("environment variable DATABASE_URL is not defined");
        let allowed_origin =
            env::var("ALLOWED_ORIGIN").expect("environment variable ALLOWED_ORIGIN is not defined");
        let public_dir = public_dir();
        let firebase_project_id = env::var("FIREBASE_PROJECT_ID")
            .expect("environment variable FIREBASE_PROJECT_ID is not defined");

        Config {
            database_url,
            allowed_origin,
            public_dir,
            firebase_project_id,
        }
    }
}

fn public_dir() -> String {
    let mut current_dir = std::env::current_dir().expect("couldn't get current directory.");
    current_dir.push("public");
    current_dir
        .into_os_string()
        .into_string()
        .expect("coudn't covert public directory to String.")
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_env());
