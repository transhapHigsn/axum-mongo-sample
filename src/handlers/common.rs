use axum::{http::StatusCode, response::IntoResponse};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World! from Axum"
}
