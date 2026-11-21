use super::handlers;
use crate::app_state::AppState;

use utoipa_axum::{routes, router::OpenApiRouter};

pub fn z_page_openapi_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new() 
        .routes(routes!(handlers::get_healthz))
}  