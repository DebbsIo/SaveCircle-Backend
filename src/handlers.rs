use actix_web::{HttpResponse, Result};

pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Welcome to SaveCircle Backend API"))
}

pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    })))
}