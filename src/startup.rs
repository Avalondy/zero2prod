use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use super::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .route("/", web::get().to(routes::greet))
            .route("/{name}", web::get().to(routes::greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
