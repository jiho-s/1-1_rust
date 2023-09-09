use actix_web::{App, HttpServer, Responder, web};
use sea_orm::Database;

use jiho_web::configuration::get_configuration;
use jiho_web::telemetry::{get_subscriber, init_subscriber};
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("jiho-todo".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let db = Database::connect(configuration.database.option()).await
        .expect("Failed to connect database.");
    Migrator::up(&db, None).await.unwrap();


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(greet))
    })
        .bind("127.0.0.1:10004")?
        .run()
        .await
}

async fn greet() -> impl Responder {
    format!("Hello world!")
}
