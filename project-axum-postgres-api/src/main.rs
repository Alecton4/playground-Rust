use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/healthchecker", get(health_check_handler));

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
