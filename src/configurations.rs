use std::convert::Infallible;

use actix_web::cookie::time::format_description::well_known::iso8601::Config;
use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("configuration"))
        .expect("Error with config");

    let op: Result<Settings, ConfigError> = settings.try_into();

    op
}
