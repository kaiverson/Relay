use axum::http::StatusCode;

// Maps any error into a '500 Internal Server Error'.
pub fn internal_error<E>(err: E) -> (StatusCode, String) 
    where 
        E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, format!("500 INTERNAL SERVER ERROR: {}", err.to_string()))
}