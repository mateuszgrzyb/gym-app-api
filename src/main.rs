use std::sync::Arc;

use axum::Server;
use sqlx::postgres::PgPoolOptions;

use gymapp::{
    api::{get_router, AppData},
    env_vars::ENV_VARS,
};

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or(ENV_VARS.log_level.as_str()),
    );

    let db = PgPoolOptions::new()
        .connect(&ENV_VARS.database_url)
        .await
        .unwrap_or_else(|e| panic! {"{}", e});

    let app_data = Arc::new(AppData { db });

    log::info!(
        "Starting web server on host {} and port {}",
        ENV_VARS.host,
        ENV_VARS.port,
    );

    let router = get_router(app_data);

    Server::bind(&ENV_VARS.get_addr().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
