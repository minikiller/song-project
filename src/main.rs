use axum::{extract::Extension, http::Method, routing::get, routing::post, Router};
use dotenv::dotenv;
use mydata::handler;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, time::Duration};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod jwt;
mod model;
mod mydata {
    pub mod handler;
}
 
static KEYS: Lazy<jwt::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    jwt::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "song_project=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:sunlf@127.0.0.1:5432/postgres".to_string());

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/", get(handler::hello_index))
        .route(
            "/bodys",
            get(handler::get_all_body_hander).post(handler::create_body_hander),
        )
        .route(
            "/bodys/:device",
            get(handler::get_body_hander)
                .patch(handler::body_update)
                .delete(handler::body_delete),
        )
        .route("/authorize", post(jwt::authorize))
        .route("/protected", get(handler::protected))
        .layer(TraceLayer::new_for_http())
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            CorsLayer::new()
                .allow_origin(Any)
                // .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
                .allow_methods(vec![Method::GET, Method::POST]),
        )
        .layer(Extension(pool));

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // song_project::foo();
    // first::good();
}
