use crate::shared_state::*;
use axum::{Json, extract::State};
use serde::Serialize;
use utoipa;

#[derive(Debug, Serialize)]
pub struct GetReadyzResponse {
    status: String,
}

#[utoipa::path(get, path = "/readyz")]
pub async fn get_readyz_handle(State(state): State<SharedState>) -> Json<GetReadyzResponse> {
    Json (GetReadyzResponse {
        status: "Endpoint not implimented!".to_string(),
    })
} 