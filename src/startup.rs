use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use routes::{health_check::health, subscriptions::subscribe};
use sqlx::{PgConnection, PgPool};
use std::net::TcpListener;

use crate::routes;

pub async fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // creates a cloneable reference to be cloned accross all cores
    let connection = Data::new(connection);

    // move transfers ownership to closure
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
