use crate::app_state::AppState;
use crate::{
    domains::{
        events::event_openapi_router,
        z_pages::{z_page_openapi_router}
    },
    errors::AppError,
};

use axum::extract::Path;
use axum::http::{StatusCode, Uri};
use axum::{
    response::IntoResponse,
    Router
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;


pub fn create_router(shared_state: AppState) -> Router {
    // let middleware_stack = ServiceBuilder::new()
    //     .layer(HandleErrorLayer::new(handle_error))
    //     .timeout(Duration::from_secs(1800));

    let event_openapi_router = OpenApiRouter::new()
        .nest("/event", event_openapi_router());
    let (event_routes, event_openapi) = event_openapi_router.split_for_parts();
        
    let (z_page_routes, z_page_openapi) = z_page_openapi_router().split_for_parts();

    let swagger_ui_routes: Router<AppState> = SwaggerUi::new("/docs")
        .url(
            "/api-docs/event/openapi.json", 
            event_openapi,
        )
        .url(
            "/api-docs/Z-Page/openapi.json", 
            z_page_openapi,
        )
        .into();

    Router::new()
        // .route("/health", axum::routing::get(health_check))
        .merge(event_routes)
        .merge(z_page_routes)
        .merge(swagger_ui_routes)
        .fallback(fallback)
        // .fallback(fallback)
        .with_state(shared_state)
}

pub async fn fallback(uri: Uri) -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_FOUND, format!("The requested URL {uri} was not found on this server.")))
}