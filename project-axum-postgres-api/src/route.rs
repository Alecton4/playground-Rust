use axum::routing::{delete, get, patch, post};
use axum::Router;
use std::sync::Arc;

use crate::handler::{
    create_one_handler, delete_one_handler, health_check_handler, read_all_handler,
    read_one_handler, update_one_handler,
};
use crate::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_check_handler))
        .route("/api/notes", post(create_one_handler))
        .route("/api/notes", get(read_all_handler))
        .route("/api/notes/:id", get(read_one_handler))
        .route("/api/notes/:id", patch(update_one_handler))
        .route("/api/notes/:id", delete(delete_one_handler))
        .with_state(app_state)
}
