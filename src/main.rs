use config::Environment;
use env_logger::Env;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, PgPool};
use std::time::Duration;
use std::{io::Stdout, net::TcpListener};
use zero2prod::configurations::get_configuration;
use zero2prod::startup;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conf = get_configuration().expect("Failed to read config");
    let address = format!("{}:{}", conf.application.host, conf.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::sink);
    init_subscriber(subscriber);

    let connection = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy_with(conf.database.with_db());

    startup::run(listener, connection).await?.await;
    Ok(())
}
