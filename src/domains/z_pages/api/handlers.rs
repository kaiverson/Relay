use crate::{
    dto::RestApiResponse,
    errors::AppError
};

use axum::response::IntoResponse;



#[utoipa::path(
    get, 
    path = "/healthz",
    responses((status = 200, description = "Get health check")), 
    tag = "Z-Page"
)]
pub async fn get_healthz(
) -> Result<impl IntoResponse, AppError> {
    Ok(RestApiResponse::success("Ok"))
}

