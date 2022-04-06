use std::time::Duration;

use anyhow::Result;
use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlb::{sqlx_exec, Field, HasFields};
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
    // init_db(&pool).await?;
    insert_data(&pool).await?;
    println!("total time is {:?}ms", start.elapsed().as_millis());
    Ok(())
}

async fn init_db(pool: &PgPool) -> Result<()> {
    sqlx::query(
        r#"
                CREATE TABLE IF NOT EXISTS todo (
                id bigserial primary key,
                title text,
                ctime timestamp with time zone default now()
                );"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_data(db_pool: &PgPool) -> Result<()> {
    let patch_data = TodoPatch {
        title: Some("Hello Title".to_string()),
    };

    // INSERT - Insert a new Todo from a Partial todo
    let sb = sqlb::insert().table("todo").data(patch_data.fields());
    let sb = sb.returning(&["id", "title"]);
    let (_id, title) = sb.fetch_one::<(i64, String), _>(db_pool).await?;

    // SELECT - Get all todos
    let sb = sqlb::select()
        .table("todo")
        .columns(&["id", "title"])
        .order_by("!id");
    let todos: Vec<Todo> = sb.fetch_all(db_pool).await?;
    assert_eq!(1, todos.len());
    Ok(())
}
#[derive(sqlx::FromRow)] // Optional: to be able to use the sqlx_exec::fetch_as...
pub struct Todo {
    pub id: i64,
    pub title: String,
}

#[derive(sqlb::Fields)] // implements sqlb::HasFields for dynamic binding
pub struct TodoPatch {
    pub title: Option<String>,
}
