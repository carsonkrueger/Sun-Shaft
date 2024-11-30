pub type ModelResult<T> = Result<T, ModelError>;

pub enum ModelError {
    Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for ModelError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}
