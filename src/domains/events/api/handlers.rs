use crate::{
    dto::RestApiResponse,
    app_state::AppState,
    errors::AppError
};

use axum::{
    extract::State,
    response::IntoResponse,
};



#[utoipa::path(
    post, 
    path = "/event",
    responses((status = 200, description = "Create event")), 
    tag = "Events"
)]
pub async fn post_event(
    State(_state): State<AppState>, 
    // Json(payload): Json<EventDto>,
) -> Result<impl IntoResponse, AppError> {
    Ok(RestApiResponse::<String>::failure(501, "Not Implemented Error"))
}



#[utoipa::path(
    get, 
    path = "/event", 
    responses((status = 200, description = "Get all events", /* body = EventListDto */)),
    tag = "Events"
)]
pub async fn get_all_events(
    State(_state): State<AppState>, 
) -> Result<impl IntoResponse, AppError> {
    if false {
        Ok("Placeholder happy path")
    } 
    else {
        Err(AppError::InternalServerError)
    }
}



#[utoipa::path(
    delete, 
    path = "/event", 
    responses((status = 200, description = "Delete all events")),
    tag = "Events",
)]
pub async fn delete_all_events(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    if false {
        Ok("Placeholder happy path")
    } 
    else {
        Err(AppError::InternalServerError)
    }
}



#[utoipa::path(
    get, 
    path = "/event/{id}",
    responses((status = 200, description = "Get event by ID", /*body = EventDto*/)),
    tag = "Events"
)]
pub async fn get_event(
    State(_state): State<AppState>, 
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if id == "succeed" {
        Ok("Placeholder happy path.")
    } 
    else {
        Err(AppError::InternalServerError)
    }
}



#[utoipa::path(
    put, 
    path = "/event/{id}",
    responses((status = 200, description = "Update event", /*body = EventDto*/)),
    tag = "Events"
)]
pub async fn put_event_update(
    State(_state): State<AppState>, 
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if id == "succeed" {
        Ok("Placeholder happy path.")
    } 
    else {
        Err(AppError::InternalServerError)
    }
}


#[utoipa::path(
    get, 
    path = "/event/{id}/deliveries",
    responses((status = 200, description = "Get event deliveries", /*body = EventDto*/)),
    tag = "Events"
)]
pub async fn get_event_deliveries(
    State(_state): State<AppState>, 
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if id == "succeed" {
        Ok("Placeholder happy path.")
    } 
    else {
        Err(AppError::InternalServerError)
    }
}
