use crate::{
    dto::ApiResponse,
    app_state::AppState,
    errors::AppError,
    domains::{
        events::dto::event_dto::{CreateEventDto, EventDto},
    }
};

use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;



#[utoipa::path(
    post, 
    path = "/event",
    responses((status = 200, description = "Create event")), 
    tag = "Events"
)]
pub async fn post_event(
    State(state): State<AppState>, 
    Json(create_event): Json<CreateEventDto>,
) -> Result<impl IntoResponse, AppError> {
    let event = state.event_service.create_event(create_event).await?;
    Ok(ApiResponse::success(event))
}



#[utoipa::path(
    get, 
    path = "/event", 
    responses((status = 200, description = "Get all events", body = Vec<EventDto>)),
    tag = "Events"
)]
pub async fn get_all_events(
    State(state): State<AppState>, 
) -> Result<impl IntoResponse, AppError> {
    let events = state.event_service.get_events().await?;
    Ok(ApiResponse::success(events))
}



#[utoipa::path(
    delete, 
    path = "/event", 
    responses((status = 200, description = "Delete all events")),
    tag = "Events",
)]
pub async fn delete_all_events(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.event_service.delete_all_events().await?;
    Ok(response)
}



#[utoipa::path(
    get, 
    path = "/event/{uuid}",
    responses((status = 200, description = "Get event by UUID", body = EventDto)),
    tag = "Events"
)]
pub async fn get_event(
    State(state): State<AppState>, 
    axum::extract::Path(event_uuid): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {    
    let event = state.event_service.get_event_by_uuid(event_uuid).await?;
    Ok(ApiResponse::success(event))
}



#[utoipa::path(
    delete, 
    path="/event/{uuid}",
    responses((status = 200, description = "Delete event", body = EventDto)),
    tag="Events"
)]
pub async fn delete_event(
    State(state): State<AppState>,
    axum::extract::Path(event_uuid): axum::extract::Path<Uuid>
) -> Result<impl IntoResponse, AppError> {
    let response = state.event_service.delete_event(event_uuid).await?;
    Ok(response)
}



#[utoipa::path(
    get, 
    path = "/event/{uuid}/deliveries",
    responses((status = 200, description = "Get event deliveries")),
    tag = "Events"
)]
pub async fn get_event_deliveries(
    State(_state): State<AppState>, 
    axum::extract::Path(uuid): axum::extract::Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    if uuid.is_max() {
        Ok("Placeholder happy path.")
    } 
    else {
        Err(AppError::InternalError)
    }
}
