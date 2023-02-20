use std::net::TcpListener;
use zero2prod::configurations::get_configuration;
use zero2prod::startup;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let conf = get_configuration().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", conf.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    startup::run(listener).await?.await
}
