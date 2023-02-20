use std::net::TcpListener;
use zero2prod::startup::run;

// tokio handles clean up of spawn under tokio::test and spins up new ones for each run
// no mem leaks
#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let resp = client
        .get(&format!("{}/health_check", &addr.await))
        .send()
        .await
        .expect("Failed to exec request");

    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length());
}

#[tokio::test]
async fn subscribe_returns_ok_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=sam%20b&email=sam_7%40gmail.com";
    let resp = client
        .post(&format!("{}/subscriptions", &app_address.await))
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
}

#[tokio::test]
async fn subscribe_returns_error_for_missing_form_data() {
    let add = spawn_app().await;
    let client = reqwest::Client::new();

    let tests = vec![
        ("name=sam%20b", "missing email"),
        ("email=sam_7%40gmail.com", "missing name"),
        ("", "missing both email and name"),
    ];

    for (body, msg) in tests {
        let resp = client
            .post(&format!("{}/subscriptions", &add))
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

async fn spawn_app() -> String {
    // port 0 calls the OS and OS will allocate an unused port for us
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let sv = run(listener).await.expect("Failed to bind address");

    let _ = tokio::spawn(sv);

    format!("http://127.0.0.1:{}", port)
}
