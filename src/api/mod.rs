use std::sync::Arc;

use axum::Router;
use axum_extra::routing::RouterExt;
use sqlx::PgPool;

pub mod form_request;
pub mod generate_key;
pub mod home;
pub mod validate_key;

pub struct AppData {
    pub db: PgPool,
}

pub fn get_router(app_data: Arc<AppData>) -> Router {
    Router::new()
        .typed_get(home::handler)
        .typed_post(form_request::handler)
        .typed_post(generate_key::handler)
        .typed_get(validate_key::handler)
        .with_state(app_data)
}
