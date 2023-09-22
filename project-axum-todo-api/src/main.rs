use axum::response::IntoResponse;
use axum::routing::get;

mod model;

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API is up and running!";
    let json_response = serde_json::json!(
        {
            "status": "success",
            "message": MESSAGE,
        }
    );
    axum::Json(json_response)
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new() // Create a new router instance.
        .route("/api/healthcheker", get(health_checker_handler)); // Specify that the health checker endpoint should only respond to GET requests.

    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap()) // Bind the HTTP server to the address.
        .serve(app.into_make_service()) // Start serving requests on that port.
        .await
        .unwrap();
}
