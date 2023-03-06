use config::Environment;
use env_logger::Env;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use std::time::Duration;
use zero2prod::configurations::get_configuration;
use zero2prod::routes::email_client::EmailClient;
use zero2prod::startup::{self, Application};
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read config");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
