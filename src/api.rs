use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::{GenerateKey, Service, ValidateKey};

#[derive(Deserialize, PartialEq, Eq)]
struct GenerateKeyInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct GenerateKeyOutput {
    key: Option<Uuid>,
}

async fn generate_key(
    State(data): State<Arc<AppData>>,
    Json(input): Json<GenerateKeyInput>,
) -> Result<Json<GenerateKeyOutput>, StatusCode> {
    let service = GenerateKey {
        username: &input.username,
        password: &input.password,
    };

    let Ok(key) = service.run(&data).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Json(GenerateKeyOutput { key }))
}

#[derive(Serialize)]
struct ValidatedKeyOutput {
    is_valid: bool,
}

async fn validate_key(
    State(data): State<Arc<AppData>>,
    Path(key): Path<Uuid>,
) -> Result<Json<ValidatedKeyOutput>, StatusCode> {
    let service = ValidateKey { key };

    let Ok(is_valid) = service.run(&data).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Json(ValidatedKeyOutput { is_valid }))
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'input> {
    endpoint: &'input str,
}

async fn index() -> Result<Html<String>, StatusCode> {
    let Ok(body) = Index {
        endpoint: "/form-request",
    }.render() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Html(body))
}

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "key_success.html")]
struct KeySuccess<'input> {
    key: &'input str,
}

#[derive(Template)]
#[template(path = "key_failure.html")]
struct KeyFailure<'input> {
    message: &'input str,
}

async fn process_form_request(
    State(data): State<Arc<AppData>>,
    Form(form_data): Form<FormData>,
) -> Result<Html<String>, StatusCode> {
    let service = GenerateKey {
        username: &form_data.username,
        password: &form_data.password,
    };

    let body = match service.run(&data).await {
        Ok(Some(key)) => KeySuccess {
            key: &key.to_string(),
        }
        .render(),
        Ok(None) => KeyFailure {
            message: "invalid credentials",
        }
        .render(),
        Err(_) => KeyFailure {
            message: "server error",
        }
        .render(),
    };

    let Ok(body) = body else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Html(body))
}

pub struct AppData {
    pub db: PgPool,
}

pub fn get_router(app_data: Arc<AppData>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/form-request", post(process_form_request))
        .route("/generate-key", post(generate_key))
        .route("/validate-key/:key", get(validate_key))
        .with_state(app_data)
}
