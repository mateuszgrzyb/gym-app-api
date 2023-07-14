use std::sync::Arc;

use axum::{extract::State, Json};
use axum_extra::routing::TypedPath;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::services::{Service, ValidateKey};

use super::AppData;

#[derive(TypedPath, Deserialize)]
#[typed_path("/validate-key/:key")]
pub struct Path {
    pub key: Uuid,
}

#[derive(Serialize)]
pub struct Output {
    is_valid: bool,
}

pub async fn handler(
    path: Path,
    State(data): State<Arc<AppData>>,
) -> Result<Json<Output>, StatusCode> {
    let service = ValidateKey { key: path.key };

    let Ok(is_valid) = service.run(&data).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Json(Output { is_valid }))
}
