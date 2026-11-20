use super::handlers;
use crate::{
    app_state::AppState,
};
use utoipa_axum::{routes, router::OpenApiRouter};

pub fn event_openapi_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new() 
        .routes(routes!(handlers::post_event))
        .routes(routes!(handlers::get_all_events))
        .routes(routes!(handlers::delete_all_events))
        .routes(routes!(handlers::get_event))
        .routes(routes!(handlers::delete_event))
        .routes(routes!(handlers::get_event_deliveries))
}  