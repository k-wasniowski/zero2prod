use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(
    listener: TcpListener,
    pg_connection_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let pg_connection_pool = web::Data::new(pg_connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(pg_connection_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}