use std::net::TcpListener;
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sea_orm::DatabaseConnection;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    connection: DatabaseConnection,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}