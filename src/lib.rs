use std::env;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use migration::MigratorTrait;

mod entities;
mod repository;
mod services;

use repository::AppState;

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Surver is up and running")
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().body("The resource doesnot exist")
}

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    dotenvy::dotenv().ok();
    let host = env::var("HOST").unwrap_or(String::from("localhost"));
    let port = env::var("HOST").unwrap_or(String::from("5000"));
    let server_url = format!("{}:{}", host, port);

    let state = AppState::new().await;

    migration::Migrator::up(&state.db, None)
        .await
        .expect("Failed to apply migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health_check)
            .configure(services::config)
            .service(services::redirect)
            .default_service(web::route().to(not_found))
    })
    .bind(server_url)?
    .run()
    .await?;

    Ok(())
}
