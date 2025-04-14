use axum::{response::IntoResponse, http::StatusCode};

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found").into_response()
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Ok").into_response()
}