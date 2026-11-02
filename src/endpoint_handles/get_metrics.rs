use crate::shared_state::*;
use axum::{Json, extract::{Path, State}, http::header::HeaderMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetMetricsResponse {
    status: String,
}

pub async fn get_metrics_handle(State(state): State<SharedState>) -> Json<GetMetricsResponse> {
    Json (GetMetricsResponse {
        status: "Endpoint not implimented!".to_string(),
    })
} 