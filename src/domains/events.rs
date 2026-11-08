pub mod api {
    mod handlers;
    pub mod routes;
}

pub mod domain {
    pub mod service;
}

pub use api::routes::{event_routes};
pub use domain::service::{EventServiceTrait, EventService};

 