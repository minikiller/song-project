// use super::jwt::{AuthError, Claims};
// use creat::model::Body;
use axum::{extract::Extension, extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

use crate::{model::Body, jwt::{Claims, AuthError}};

// we can extract the connection pool with `Extension`
pub async fn hello_index(
    Extension(pool): Extension<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

pub async fn create_body_hander(
    Extension(pool): Extension<PgPool>,
    Json(new_body): Json<Body>,
) -> impl IntoResponse {
    let body = sqlx::query!(
        r#"INSERT INTO body 
        (device_id,content) 
        VALUES ($1,$2) 
        RETURNING id,device_id,content,time"#,
        new_body.device_id,
        new_body.content
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    tracing::debug!("{:?}", body);
    let my_body = Body {
        id: body.id,
        device_id: body.device_id,
        content: body.content,
        time: Some(NaiveDateTime::from(body.time.unwrap())),
    };
    // StatusCode::CREATED
    (StatusCode::CREATED, Json(my_body))
}

pub async fn get_all_body_hander(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let rows = sqlx::query!(
        r#"SELECT id,device_id,content,time
        FROM body"#
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let result: Vec<Body> = rows
        .iter()
        .map(|row| Body {
            device_id: row.device_id,
            id: row.id,
            content: row.content.clone(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
        .collect();
    // println!("{:?}", body);
    (StatusCode::OK, Json(result))
    // StatusCode::OK
}

pub async fn get_body_hander(
    claims: Claims,
    Path(device_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let rows = sqlx::query!(
        r#"SELECT id,device_id,content,time
        FROM body where device_id=$1"#,
        device_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    tracing::debug!("current user info is {}", claims);
    let result: Vec<Body> = rows
        .iter()
        .map(|row| Body {
            device_id: row.device_id,
            id: row.id,
            content: row.content.clone(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
        .collect();
    // println!("{:?}", body);
    (StatusCode::OK, Json(result))
    // StatusCode::OK
}

async fn update(pool: &PgPool, body: &Body, id: i32) -> Result<(), sqlx::Error> {
    let transaction = pool.begin().await?;
    sqlx::query("UPDATE body SET devide_id=? WHERE id=?")
        .bind(&body.device_id)
        .bind(id)
        .execute(pool)
        .await?;

    // 这里只关心 commit，因为 https://docs.rs/sqlx/0.5.1/sqlx/struct.Transaction.html 说到
    // If neither are called before the transaction goes out-of-scope, rollback is called. In other words, rollback is called on drop if the transaction is still in-progress.
    transaction.commit().await?;
    Ok(())
}

pub async fn body_update(
    Path(id): Path<i32>,
    Json(update_body): Json<Body>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    tracing::debug!("current body info is {:?}", update_body);
    // let transaction = pool.begin();
    let result = sqlx::query!(
        "UPDATE body SET content=$1 WHERE id=$2",
        update_body.content,
        id
    )
    // .bind(&update_body.device_id)
    // .bind(id)
    .execute(&pool)
    .await
    .unwrap();
    // transaction.commit().await;

    // let result=update(&pool, &update_body, id).await.unwrap();

    // println!("{:?}", body);
    StatusCode::OK
    // StatusCode::OK
}

pub async fn body_delete(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    let rows = sqlx::query!(r#"DELETE FROM Body where id=$1"#, id)
        .execute(&pool)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
    // StatusCode::NOT_FOUND
    // StatusCode::OK
}

pub async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
