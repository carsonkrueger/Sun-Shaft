use sea_query::enum_def;
use sqlx::types::chrono;

#[enum_def]
pub struct Items {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: chrono::NaiveDate,
}
