use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::Html, Form};
use axum_extra::routing::TypedPath;
use hyper::StatusCode;
use serde::Deserialize;

use crate::services::{GenerateKey, Service};

use super::AppData;

#[derive(TypedPath, Deserialize)]
#[typed_path("/form-request")]
pub struct Path {}

#[derive(Deserialize)]
pub struct Input {
    username: String,
    password: String,
}

#[derive(Template)]
#[template(path = "key_success.html")]
pub struct KeySuccess<'input> {
    pub key: &'input str,
}

#[derive(Template)]
#[template(path = "key_failure.html")]
pub struct KeyFailure<'input> {
    pub message: &'input str,
}

pub async fn handler(
    _: Path,
    State(data): State<Arc<AppData>>,
    Form(form_data): Form<Input>,
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
