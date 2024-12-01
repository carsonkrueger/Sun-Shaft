pub type ModelResult<T> = Result<T, ModelError>;

pub enum ModelError {
    Sqlx(sqlx::Error),
    SeaQuery(sea_query::error::Error),
}

impl From<sqlx::Error> for ModelError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<sea_query::error::Error> for ModelError {
    fn from(value: sea_query::error::Error) -> Self {
        Self::SeaQuery(value)
    }
}
