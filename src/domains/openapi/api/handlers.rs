
use axum::Json;

#[utoipa::path(
    get, 
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "Get the OpenApi documentation for the Relay API")
    )
)]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}