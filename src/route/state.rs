use sqlx::{Pool, Postgres};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}
