// This code comes from https://github.com/sukjaelee/clean_axum_demo/blob/main/src/common/dto.rs

use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T>
where 
    T: Serialize,
{
    pub status: u16, 
    pub message: String, 
    pub data: Option<T>
}


impl<T> ApiResponse<T>
where 
    T: Serialize, 
{
    pub fn success(data: T) -> Self {
        Self {
            status: 200, 
            message: "success".to_string(), 
            data: Some(data),
        }
    }

    pub fn success_with_message(message: impl Into<String>, data: T) -> Self {
        Self {
            status: 200, 
            message: message.into(), 
            data: Some(data)
        }
    }

    pub fn failure(status: u16, message: impl Into<String>) -> Self {
        Self {
            status, 
            message: message.into(), 
            data: None,
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct RestApiResponse<T: Serialize>(pub ApiResponse<T>);

impl<T: Serialize> RestApiResponse<T> {

    pub fn success(data: T) -> Self {
        Self(ApiResponse::success(data))
    }

    pub fn success_with_message(message: impl Into<String>, data: T) -> Self {
        Self(ApiResponse::success_with_message(message, data))
    }

    pub fn failure(status: u16, message: impl Into<String>) -> Self {
        Self(ApiResponse::failure(status, message))
    }
}

impl<T: Serialize> IntoResponse for RestApiResponse<T> {
    fn into_response(self) -> Response { 
        axum::Json(self.0).into_response()
    }
}