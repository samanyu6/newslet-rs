use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check::health, subscriptions::subscribe};
use std::net::TcpListener;

use crate::routes;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
