use std::net::TcpListener;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use crate::routes::{health_check, subscribe};
use sqlx::{PgPool};

pub fn run(
    listener: TcpListener,
    connection: PgPool
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
        .app_data(connection.clone())
})
        .listen(listener)?
        .run();
    Ok(server)
}