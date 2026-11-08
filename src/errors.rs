use axum::{
    Router,
    body::{self, Bytes},
    routing::get,
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Maps any error into a '500 Internal Server Error'.
pub fn internal_error<E>(err: E) -> (StatusCode, String) 
    where 
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, format!("500 INTERNAL SERVER ERROR: {}", err.to_string()))
}

pub enum AppError {
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body: &str = match self {
            AppError::InternalServerError => "INTERNAL SERVER ERROR"
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}