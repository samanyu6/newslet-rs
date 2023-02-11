use std::net::TcpListener;

use zero2prod::run;

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

async fn spawn_app() -> String {
    // port 0 calls the OS and OS will allocate an unused port for us
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let sv = run(listener).await.expect("Failed to bind address");

    let _ = tokio::spawn(sv);

    format!("http://127.0.0.1:{}", port)
}
