use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simple_dto_mapper_derive::DtoFrom;
use utoipa::ToSchema;
use validator::Validate;

use crate::domains::events::domain::model::Event;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, DtoFrom)]
#[dto(from = Event)]
pub struct EventDto {
    #[schema(value_type = String)]
    pub uuid: uuid::Uuid,
    pub event_type: String,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub payload: Value,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateEventDto {
    pub event_type: String, 
    pub payload: Value,
}