use crate::shared_state::*;
use axum::{Json, extract::{Path, State}};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetEventsIdDeliveriesResponse {
    deliveries: String,
}

pub async fn get_events_id_deliveries_handle(State(state): State<SharedState>, Path(id): Path<String>) -> Json<GetEventsIdDeliveriesResponse> {
    Json ( GetEventsIdDeliveriesResponse {
        deliveries: "endpoint not implimented".to_string(), 
    })
}