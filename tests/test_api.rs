use std::sync::Arc;

use askama::Template;
use axum_extra::routing::TypedPath;
use axum_test::TestServer;
use gymapp::{
    api::{form_request, generate_key, get_router, home, AppData},
    errors::R,
    models::Ad,
};
use macros::form;
use rstest::*;
use serde_json::{json, Value};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

fn app_data_factory(pool: PgPool) -> Arc<AppData> {
    Arc::new(AppData { db: pool })
}

fn server_factory(pool: PgPool) -> R<TestServer> {
    Ok(TestServer::new(
        get_router(app_data_factory(pool)).into_make_service(),
    )?)
}

#[sqlx::test]
#[rstest]
async fn test_generate_key(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;

    // when
    let response = server
        .post(generate_key::Path::PATH)
        .json(&json!(
            {
                "username": "admin",
                "password": "admin"
            }
        ))
        .await;

    // then
    let response_json = response.json::<Value>();
    let response_key = response_json["key"].as_str().unwrap();

    let expected_key = query_as::<_, Ad>(r#"SELECT * FROM "ad""#)
        .fetch_one(&pool)
        .await?
        .key;
    assert_eq!(Uuid::parse_str(response_key)?, expected_key);

    Ok(())
}

#[sqlx::test]
#[rstest]
async fn test_home(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;

    // when
    let response = server.get(home::Path::PATH).await;

    // then
    let response_body = response.text();
    let expected_body = home::Home {
        endpoint: "/form-request",
    }
    .render()?;

    assert_eq!(response_body, expected_body);

    Ok(())
}

struct Admin {}

impl ToString for Admin {
    fn to_string(&self) -> String {
        "admin".into()
    }
}

#[sqlx::test]
#[rstest]
async fn test_form_request_success(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;

    // when
    let response = server
        .post(form_request::Path::PATH)
        .content_type("application/x-www-form-urlencoded")
        .text(form! {
            "username": "admin",
            "password": "admin"
        })
        .await;

    // then
    let response_body = response.text();

    let key = query_as::<_, Ad>(r#"SELECT * FROM "ad""#)
        .fetch_one(&pool)
        .await?
        .key;
    let expected_body = form_request::KeySuccess {
        key: key.to_string().as_str(),
    }
    .render()?;

    assert_eq!(response_body, expected_body);

    Ok(())
}

#[sqlx::test]
#[rstest]
async fn test_form_request_failure(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;

    // when
    let response = server
        .post(form_request::Path::PATH)
        .content_type("application/x-www-form-urlencoded")
        .text(form! {
            "username": "wrong",
            "password": "credentials"
        })
        .await;

    // then
    let response_body = response.text();

    let expected_body = form_request::KeyFailure {
        message: "invalid credentials",
    }
    .render()?;

    assert_eq!(response_body, expected_body);

    Ok(())
}
