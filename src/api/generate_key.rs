use std::sync::Arc;

use axum::{extract::State, Json};
use axum_extra::routing::TypedPath;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::{GenerateKey, Service};

use super::AppData;

#[derive(TypedPath, Deserialize)]
#[typed_path("/generate-key")]
pub struct Path {}

#[derive(Deserialize, PartialEq, Eq)]
pub struct Input {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct Output {
    key: Option<Uuid>,
}

pub async fn handler(
    _: Path,
    State(data): State<Arc<AppData>>,
    Json(input): Json<Input>,
) -> Result<Json<Output>, StatusCode> {
    let service = GenerateKey {
        username: &input.username,
        password: &input.password,
    };

    let Ok(key) = service.run(&data).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Json(Output { key }))
}
