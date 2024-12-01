use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use steamworks::AppId;

pub type RouteResult<T> = Result<T, RouteError>;

pub enum RouteError {
    SAInitError(steamworks::SteamAPIInitError),
    SAAppNotInstalled(AppId),
    MissingField(String),
    Multipart(axum::extract::multipart::MultipartError),
    IO(std::io::Error),
    SeaQuery(sea_query::error::Error),
    Sqlx(sqlx::error::Error),
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        match self {
            RouteError::SAInitError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::IO(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            RouteError::SAAppNotInstalled(id) => (
                StatusCode::BAD_REQUEST,
                format!("App {} not installed", id.0),
            ),
            RouteError::MissingField(f) => (StatusCode::INTERNAL_SERVER_ERROR, f),
            RouteError::Multipart(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.to_string()),
            RouteError::SeaQuery(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            RouteError::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
        .into_response()
    }
}

impl From<steamworks::SteamAPIInitError> for RouteError {
    fn from(value: steamworks::SteamAPIInitError) -> Self {
        Self::SAInitError(value)
    }
}

impl From<std::io::Error> for RouteError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<axum::extract::multipart::MultipartError> for RouteError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        Self::Multipart(value)
    }
}

impl From<sea_query::error::Error> for RouteError {
    fn from(value: sea_query::error::Error) -> Self {
        Self::SeaQuery(value)
    }
}

impl From<sqlx::error::Error> for RouteError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::Sqlx(value)
    }
}
