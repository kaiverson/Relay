use super::handlers;
use crate::{
    app_state::AppState,
};
use axum::{
    routing::{get, post, put, delete}, 
    Router
};

pub fn event_routes() -> Router<AppState> {
    Router::new() 
        .route("/", post(handlers::post_event))
        .route("/", get(handlers::get_all_events))
        .route("/", delete(handlers::delete_all_events))
        .route("/{id}", get(handlers::get_event))
        .route("/{id}", put(handlers::put_event_update))
        .route("/{id}/deliveries", get(handlers::get_event_deliveries))
}  