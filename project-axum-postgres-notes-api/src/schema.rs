use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
}
