use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

use crate::model::NoteModel;
use crate::schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema};
use crate::AppState;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });
    Json(json_response)
}

pub async fn create_one_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title, content, category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let json_response = serde_json::json!(
                {
                    "status": "success",
                    "data": serde_json::json!(
                        {
                            "note": note
                        }
                    )
                }
            );
            Ok((StatusCode::CREATED, Json(json_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!(
                    {
                        "status": "fail",
                        "message": "Note with that title already exists!"
                    }
                );
                println!("❌ Returning error response...");
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            println!("❌ Returning error response...");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!(
                    {
                        "status": "error",
                        "message": format!("{:?}", e)
                    }
                )),
            ))
        }
    }
}

pub async fn read_all_handler(
    State(data): State<Arc<AppState>>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!(
            {
                "status": "fail",
                "message": "Something went wrong while fetching all notes!"
            }
        );
        println!("❌ Returning error response...");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let notes = query_result.unwrap();
    let json_response = serde_json::json!(
        {
            "status": "success",
            "results": notes.len(),
            "notes": notes
        }
    );
    Ok(Json(json_response))
}

pub async fn read_one_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!(
                {
                    "status": "success",
                    "data": serde_json::json!(
                        {
                            "note": note
                        }
                    )
                }
            );
            Ok(Json(note_response))
        }
        Err(_) => {
            let error_response = serde_json::json!(
                {
                    "status": "fail",
                    "message": format!("Note with id {} not found!", id)
                }
            );
            println!("❌ Returning error response...");
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn update_one_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!(
            {
                "status": "fail",
                "message": format!("Note with id {} not found!", id)
            }
        );
        println!("❌ Returning error response...");
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let note = query_result.unwrap();
    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        id
    ).fetch_one(&data.db).await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!(
                {
                    "status": "success",
                    "data": serde_json::json!(
                        {
                            "note": note
                        }
                    )
                }
            );
            Ok(Json(note_response))
        }
        Err(e) => {
            println!("❌ Returning error response...");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!(
                    {
                        "status": "error",
                        "message": format!("{:?}", e)
                    }
                )),
            ))
        }
    }
}

pub async fn delete_one_handler(
    State(data): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!(
            {
                "status": "fail",
                "message": format!("Note with id {} not found!", id)
            }
        );
        println!("❌ Returning error response...");
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}
