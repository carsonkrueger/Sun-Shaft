use sea_query::enum_def;
use sqlx::{types::chrono, FromRow};

#[derive(FromRow, Debug)]
#[enum_def]
pub struct SunUsers {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}
