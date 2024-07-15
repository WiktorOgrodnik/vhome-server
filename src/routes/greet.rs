use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(serde::Serialize)]
struct Message {
    content: String,
}

pub async fn default() -> impl IntoResponse {
    let content = "Welcome to grouplist API!".to_owned();
    let message = Message { content };

    (StatusCode::OK, Json(message))
}
