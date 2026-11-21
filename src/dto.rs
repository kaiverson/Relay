use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;

#[derive(Debug)]
pub struct ApiResponse<T>
where 
    T: Serialize,
{
    pub status: StatusCode, 
    pub message: String, 
    pub data: Option<T>
}

#[derive(Serialize)]
struct ApiResponseBody<T: Serialize> {
    message: String,
    data: Option<T>,
}

impl<T> ApiResponse<T>
where 
    T: Serialize, 
{
    pub fn success(data: T) -> Self {
        Self {
            status: StatusCode::OK, 
            message: "success".to_string(), 
            data: Some(data),
        }
    }

    pub fn success_with_message(message: impl Into<String>, data: T) -> Self {
        Self {
            status: StatusCode::OK, 
            message: message.into(), 
            data: Some(data)
        }
    }

    pub fn failure(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status, 
            message: message.into(), 
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response { 
        let body = ApiResponseBody {
            message: self.message, 
            data: self.data
        };
        (self.status, axum::Json(body)).into_response()
    }
}