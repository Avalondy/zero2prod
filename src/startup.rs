use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool in a smart pointer (Arc)
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // Add the logger middleware
            .wrap(Logger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            // Get a pointer copy and attach it to the application state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
