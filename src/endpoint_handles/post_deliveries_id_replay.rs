use crate::shared_state::*;
use axum::{Json, extract::{Path, State}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PostDeliveriesIdReplayRequest {}

#[derive(Debug, Serialize)]
pub struct PostDeliveriesIdReplayResponse {
    deliveries: String,
}

pub async fn post_deliveries_id_replay_handle(
    State(state): State<SharedState>, 
    Path(id): Path<String>, 
    Json(request): Json<PostDeliveriesIdReplayRequest>,
) -> Json<PostDeliveriesIdReplayResponse> {
    Json ( PostDeliveriesIdReplayResponse {
        deliveries: "endpoint not implimented".to_string(), 
    })
}