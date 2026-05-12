use actix_web::{web, App, HttpServer, Result};
use std::io::Result as IoResult;

mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> IoResult<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // TODO: Set up database connection pool with Diesel

    println!("Starting SaveCircle Backend server on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(handlers::index))
            .route("/health", web::get().to(handlers::health))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}