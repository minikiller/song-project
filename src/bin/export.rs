use std::time::Duration;

use anyhow::Result;
use chrono::NaiveDateTime;
use dotenv::dotenv;
use song_project::model::{Body, Person};
use sqlx::{postgres::PgPoolOptions, PgPool};
///导出csv文件到pg数据库
///
///
#[tokio::main]
async fn main() -> Result<()> {
    let start = std::time::Instant::now();
    dotenv().ok();
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:sunlf@127.0.0.1:5432/postgres".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");
    // get_data(&pool).await?;
    let db = vec![Person::new(50001, "hello"), Person::new(50002, "world")];
    insert_data(&pool, db).await?;
    println!("total time is {:?}ms", start.elapsed().as_millis());
    Ok(())
}

async fn get_data(pool: &PgPool) -> Result<()> {
    let rows = sqlx::query!(
        r#"SELECT id,device_id,content,time
        FROM body"#
    )
    .fetch_all(pool)
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

    dbg!(result);
    Ok(())
}

async fn insert_data(pool: &PgPool, persons: Vec<Person>) -> Result<()> {
    for person in persons {
       let body =  sqlx::query!(
            r#"INSERT INTO tb_card 
        (id,card) 
        VALUES ($1,$2) 
        RETURNING id,card,time"#,
            person.id,
            person.card
        )
        .fetch_one(pool)
        .await
        .unwrap();
        dbg!(body);
    }
    Ok(())
}
