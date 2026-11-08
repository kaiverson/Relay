use super::handlers;
use crate::app_state::AppState;
use axum::{
    routing::get,
    Router
};

pub fn z_page_routes() -> Router<AppState> {
    Router::new() 
        .route("/", get(handlers::get_healthz))
}  