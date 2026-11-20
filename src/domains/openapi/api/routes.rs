use crate::app_state::AppState;
use super::handlers;
use utoipa_axum::{routes, router::OpenApiRouter};

pub fn openapi_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new() 
        .routes(routes!(handlers::openapi))
}  