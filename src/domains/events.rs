pub mod api {
    mod handlers;
    pub mod routes;
}

pub mod domain {
    pub mod model;
    pub mod repository;
    pub mod service;
}

pub mod dto {
    pub mod event_dto;
}

mod infra {
    mod impl_repository;
    pub mod impl_service;
}

pub use api::routes::{event_routes};
pub use domain::service::EventServiceTrait;
pub use infra::impl_service::EventService;
 