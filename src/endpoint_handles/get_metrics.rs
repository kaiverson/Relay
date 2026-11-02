use crate::shared_state::*;
use axum::{Json, extract::State};
use serde::Serialize;
use utoipa;

#[derive(Debug, Serialize)]
pub struct GetMetricsResponse {
    status: String,
}

#[utoipa::path(get, path = "/metrics")]
pub async fn get_metrics_handle(State(state): State<SharedState>) -> Json<GetMetricsResponse> {
    Json (GetMetricsResponse {
        status: "Endpoint not implimented!".to_string(),
    })
} 