use std::sync::{Arc, RwLock};
use std::{collections::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    pub name: String
}

pub struct AppState {
    pub db: RwLock<HashMap<String, Event>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {db: RwLock::new(HashMap::new())}
    }
}

pub type SharedState = Arc<AppState>;
