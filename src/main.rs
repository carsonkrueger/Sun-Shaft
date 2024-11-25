use config::Config;
use dotenvy::dotenv;
use routes::AppState;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

mod config;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().expect("could not load .env");
    let config: Config = Config::load_config().expect("Err loading config");
    let pool = create_pool(&config).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Could not run migrations");

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let state = AppState { pool };
    let router = routes::create_routes(state).layer(cors);

    let addr = format!("{}:{}", config.BACK_END_DOMAIN, config.BACK_END_PORT);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Could not listen on {}", addr));

    println!("Serving on {}", addr);
    axum::serve(listener, router)
        .await
        .expect("Could not serve axum app");
}

async fn create_pool(config: &Config) -> Pool<Postgres> {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.DB_USER, config.DB_PASSWORD, config.DB_DOMAIN, config.DB_PORT, config.DB_NAME
    );

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");
    pool
}
