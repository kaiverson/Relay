use crate::app_state::AppState;

use axum::{
    Router,
};


// use utoipa::openapi::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

use crate::{
    domains::{
        events::{event_routes,},
        z_pages::{z_page_routes}
    }
};


// fn _create_swagger_ui(doc: OpenApi) -> SwaggerUi {
//     SwaggerUi::new("/docs")
//         .url(
//             "/api-docs/user_auth/openapi.json", 
//             doc,
//         )
// }

pub fn create_router(shared_state: AppState) -> Router {
    // let middleware_stack = ServiceBuilder::new()
    //     .layer(HandleErrorLayer::new(handle_error))
    //     .timeout(Duration::from_secs(1800));



    // let router = OpenApiRouter::with_openapi(ApiDoc::openapi())
    //     .routes(routes!(post_events::post_events_handle))
    //     .routes(routes!(get_events_id::get_events_id_handle))
    //     .routes(routes!(get_events_id_deliveries::get_events_id_deliveries_handle))
    //     .routes(routes!(post_deliveries_id_replay::post_deliveries_id_replay_handle))
    //     .routes(routes!(get_healthz::get_healthz_handle))
    //     .routes(routes!(get_readyz::get_readyz_handle))
    //     .routes(routes!(get_metrics::get_metrics_handle));
    
    let event_routes = Router::new()
        .nest("/events", event_routes());
        // .route_layer(middleware::from_fn(jwt::jwt_auth));

    let z_page_routes = Router::new()
        .merge(z_page_routes());

    Router::new()
        // .route("/health", axum::routing::get(health_check))
        .merge(event_routes)
        .merge(z_page_routes)
        // .fallback(fallback)
        .with_state(shared_state)
}