use dotenvy::dotenv;
use routes::AppState;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

// mod libs;
// mod middleware;
// mod models;
mod routes;
// mod services;

pub static mut ENVIRONMENT: Environment = Environment::DEV;

#[tokio::main]
async fn main() {
    dotenv().expect("could not load .env");
    let pool = create_pool().await;

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

    let be_domain = dotenvy::var("BACK_END_DOMAIN").expect("BE domain missing");
    let be_port = dotenvy::var("BACK_END_PORT").expect("BE port missing");
    let addr = format!("{}:{}", be_domain, be_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Could not listen on {}", addr));

    println!("Serving on {}", addr);
    axum::serve(listener, router)
        .await
        .expect("Could not serve axum app");
}

async fn create_pool() -> Pool<Postgres> {
    let environment = dotenvy::var(&"ENVIRONMENT").expect("Environment var missing");
    let db_name = dotenvy::var(&"DB_NAME").expect("DB name missing");
    let db_user = dotenvy::var(&"DB_USER").expect("DB user missing");
    let db_domain = dotenvy::var(&"DB_DOMAIN").expect("DB domain missing");
    let db_port = dotenvy::var(&"DB_PORT").expect("DB port missing");
    let db_password = dotenvy::var(&"DB_PASSWORD").expect("DB password missing");

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_domain, db_port, db_name
    );

    unsafe {
        ENVIRONMENT = Environment::from(environment);
    }

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");
    pool
}

pub enum Environment {
    DEV,
    PROD,
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "production" | "prod" => Self::PROD,
            "development" | "dev" | _ => Self::DEV,
        }
    }
}
