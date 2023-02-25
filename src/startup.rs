use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use routes::{health_check::health, subscriptions::subscribe};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::routes;

pub async fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // creates a cloneable reference to be cloned accross all cores
    let connection = Data::new(connection);

    // move transfers ownership to closure
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
