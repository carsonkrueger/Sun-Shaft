use sea_query::enum_def;
use sqlx::types::chrono;

#[enum_def]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDate,
}
