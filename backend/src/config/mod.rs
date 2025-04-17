use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: AppConfig = AppConfig::from_env();
}

pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub app_host: String,
    pub app_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            app_host: env::var("APP_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| String::from("8000"))
                .parse()
                .expect("APP_PORT must be a valid number"),
        }
    }
}