use crate::shared_state::*;
use axum::{Json, extract::{Path, State}, http::header::HeaderMap};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize)]
// struct GetEventsIdRequest {
//     event_type: String, 
//     payload: serde_json::Value,
// }

#[derive(Debug, Serialize)]
pub struct GetHealthzResponse {
    status: String,
}

pub async fn get_healthz_handle(State(state): State<SharedState>) -> Json<GetHealthzResponse> {
    Json (GetHealthzResponse {
        status: "Healthy!".to_string(),
    })
} 