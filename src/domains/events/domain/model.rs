use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Event {
    pub uuid: Uuid,
    pub event_type: String, 
    pub created_at: DateTime<Utc>, 
    pub payload: Value,
}