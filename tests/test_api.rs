use std::sync::Arc;

use askama::Template;
use axum_extra::routing::TypedPath;
use axum_test::TestServer;
use gymapp::{
    api::{form_request, generate_key, get_router, home, validate_key, AppData},
    errors::R,
    models::Ad,
};
use macros::form;
use rstest::*;
use serde_json::json;
use sqlx::{query, query_as, PgPool};
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
async fn test_generate_key_success(pool: PgPool) -> R<()> {
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
    let expected_key = query_as::<_, Ad>(r#"SELECT * FROM "ad""#)
        .fetch_one(&pool)
        .await?
        .key;

    response.assert_status_ok();
    response.assert_json(&json!({ "key": expected_key }));

    Ok(())
}

#[sqlx::test]
#[rstest]
async fn test_generate_key_failure(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;

    // when
    let response = server
        .post(generate_key::Path::PATH)
        .json(&json!(
            {
                "username": "wrong",
                "password": "credentials"
            }
        ))
        .await;

    // then
    response.assert_status_ok();
    response.assert_json(&json!({ "key": null }));

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
    response.assert_status_ok();
    response.assert_text(
        home::Home {
            endpoint: "/form-request",
        }
        .render()?,
    );

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
    let key = &query_as::<_, Ad>(r#"SELECT * FROM "ad""#)
        .fetch_one(&pool)
        .await?
        .key
        .to_string();

    response.assert_status_ok();
    response.assert_text(form_request::KeySuccess { key }.render()?);

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
    response.assert_status_ok();
    response.assert_text(
        form_request::KeyFailure {
            message: "invalid credentials",
        }
        .render()?,
    );

    Ok(())
}

#[sqlx::test]
#[rstest]
async fn test_validate_key_success(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;
    let key = Uuid::new_v4();
    query(r#"INSERT INTO "ad" ("key") VALUES ($1)"#)
        .bind(key)
        .execute(&pool)
        .await?;

    // when
    let response = server.get(&validate_key::Path { key }.to_string()).await;

    // then
    response.assert_status_ok();
    response.assert_json(&json!(
        {
            "is_valid": true
        }
    ));

    Ok(())
}

#[sqlx::test]
#[rstest]
async fn test_validate_key_failure(pool: PgPool) -> R<()> {
    // given
    let server = server_factory(pool.clone())?;
    let key = Uuid::new_v4();

    // when
    let response = server.get(&validate_key::Path { key }.to_string()).await;

    // then
    response.assert_status_ok();
    response.assert_json(&json!(
        {
            "is_valid": false
        }
    ));

    Ok(())
}
