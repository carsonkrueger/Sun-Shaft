use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}
