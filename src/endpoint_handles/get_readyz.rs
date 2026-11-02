use crate::shared_state::*;
use axum::{Json, extract::{Path, State}, http::header::HeaderMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GetReadyzResponse {
    status: String,
}

pub async fn get_readyz_handle(State(state): State<SharedState>) -> Json<GetReadyzResponse> {
    Json (GetReadyzResponse {
        status: "Endpoint not implimented!".to_string(),
    })
} 