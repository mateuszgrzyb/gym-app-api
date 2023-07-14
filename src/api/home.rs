use askama::Template;
use axum::response::Html;
use axum_extra::routing::TypedPath;
use hyper::StatusCode;
use serde::Deserialize;

use super::form_request;

#[derive(TypedPath, Deserialize)]
#[typed_path("/")]
pub struct Path {}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Home<'input> {
    pub endpoint: &'input str,
}

pub async fn handler(_: Path) -> Result<Html<String>, StatusCode> {
    let Ok(body) = Home {
        endpoint: form_request::Path::PATH,
    }.render() else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    Ok(Html(body))
}
