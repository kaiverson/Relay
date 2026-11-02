use crate::shared_state::*;
use axum::{Json, extract::State};
use serde::Serialize;
use utoipa;

#[derive(Debug, Serialize)]
pub struct GetHealthzResponse {
    status: String,
}

#[utoipa::path(get, path = "/healthz")]
pub async fn get_healthz_handle(State(state): State<SharedState>) -> Json<GetHealthzResponse> {
    Json (GetHealthzResponse {
        status: "Healthy!".to_string(),
    })
} 