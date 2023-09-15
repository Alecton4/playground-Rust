use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

mod handler;
mod model;
mod schema;

pub struct AppState {
    db: Pool<Postgres>,
}

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
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = Router::new()
        .route("/api/healthchecker", get(health_check_handler))
        .with_state(app_state);
    println!("🚀 Server started successfully");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
