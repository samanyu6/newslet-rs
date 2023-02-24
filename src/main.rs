use config::Environment;
use env_logger::Env;
use sqlx::{Connection, PgConnection, PgPool};
use std::{io::Stdout, net::TcpListener};
use zero2prod::configurations::get_configuration;
use zero2prod::startup;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conf = get_configuration().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", conf.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");

    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let connection = PgPool::connect(&conf.database.connection_string())
        .await
        .expect("Failed to connect to the db");

    startup::run(listener, connection).await?.await;
    Ok(())
}
