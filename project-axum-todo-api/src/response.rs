use serde::Serialize;

use crate::model::Task;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TaskData {
    pub task: Task,
}

#[derive(Debug, Serialize)]
pub struct SingleTaskResponse {
    pub status: String,
    pub data: TaskData,
}

#[derive(Debug, Serialize)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub data: Vec<TaskData>,
}
