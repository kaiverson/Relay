use std::sync::{Arc, RwLock};

pub struct AppState {
    pub log: RwLock<Vec<String>>,
}

pub type SharedState = Arc<AppState>;
