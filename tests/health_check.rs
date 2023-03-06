use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{postgres::PgRow, Connection, Executor, PgConnection, PgPool, Row};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::{
    configurations::{get_configuration, DatabaseSettings},
    routes::{email_client::EmailClient, FormData},
    startup::{get_connection_pool, run, Application},
    telemetry::{get_subscriber, init_subscriber},
};
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
// tokio handles clean up of spawn under tokio::test and spins up new ones for each run
// no mem leaks
#[tokio::test]
async fn health_check_works() {
    let test_app = spawn_app();
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/health_check", &test_app.await.address))
        .send()
        .await
        .expect("Failed to exec request");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscribe_returns_ok_for_valid_form_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let configuration = get_configuration().expect("Failed to reac config");

    let body = "name=sam%20b&email=sam7%40gmail.com";
    let resp = client
        .post(&format!("{}/subscriptions", &test_app.address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to exec request");

    assert_eq!(
        200,
        resp.status().as_u16(),
        "API failed with correct input {}",
        body
    );

    let saved = sqlx::query("SELECT email, name FROM subscriptions")
        .map(|row: PgRow| FormData {
            email: row.get("email"),
            name: row.get("name"),
        })
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch users");
    // println!("{:?}", saved);
    assert_eq!(saved.name, "sam b");
    assert_eq!(saved.email, "sam7@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_error_for_missing_form_data() {
    let add = spawn_app().await;
    let client = reqwest::Client::new();

    let tests = vec![
        ("name=sam%20b", "missing email"),
        ("email=sam7%40gmail.com", "missing name"),
        ("", "missing both email and name"),
    ];

    for (body, msg) in tests {
        let resp = client
            .post(&format!("{}/subscriptions", &add.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            resp.status().as_u16(),
            "API did not fail with 400 when a bad request {} was sent",
            msg
        );
    }
}

// can't call init_subscriber more than once in our tests, we want only one exec
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

async fn spawn_app() -> TestApp {
    // first init makes it launch tracing, all other invocations skip exec
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");

        c.database.database_name = Uuid::new_v4().to_string();

        c.application.port = 0;

        c
    };

    configure_database(&configuration.database).await;
    let app = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let address = format!("http://127.0.0.1:{}", app.port());

    let _ = tokio::spawn(app.run_until_stopped());

    TestApp {
        address: address,
        db_pool: get_connection_pool(&configuration.database),
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connection without db_name");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Database create failed");

    // migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to create connection pool");

    sqlx::migrate!()
        .run(&connection_pool)
        .await
        .expect("Failed to migrate db");

    connection_pool
}
