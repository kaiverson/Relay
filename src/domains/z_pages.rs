pub mod api {
    mod handlers;
    pub mod routes;
}

pub use api::routes::{z_page_openapi_router};
 