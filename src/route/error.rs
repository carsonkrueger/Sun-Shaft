use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use steamworks::AppId;

pub type RouteResult<T> = Result<T, RouteError>;

#[derive(Debug, Clone)]
pub enum RouteError {
    SteamAPIInit(String),
    SAAppNotInstalled(AppId),
    MissingField(String),
    Multipart(String),
    IO(String),
    SeaQuery(String),
    Sqlx(String),
    InvalidAuth,
    MissingAuthCookie,
    JWT(String),
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        match self {
            RouteError::SteamAPIInit(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            Self::IO(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            RouteError::SAAppNotInstalled(id) => (
                StatusCode::BAD_REQUEST,
                format!("App {} not installed", id.0),
            ),
            RouteError::MissingField(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            RouteError::Multipart(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            RouteError::SeaQuery(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            RouteError::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            RouteError::JWT(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            RouteError::InvalidAuth => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid Auth".to_string(),
            ),
            RouteError::MissingAuthCookie => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Missing Auth Cookie".to_string(),
            ),
        }
        .into_response()
    }
}

impl From<steamworks::SteamAPIInitError> for RouteError {
    fn from(value: steamworks::SteamAPIInitError) -> Self {
        Self::SteamAPIInit(value.to_string())
    }
}

impl From<std::io::Error> for RouteError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value.to_string())
    }
}

impl From<axum::extract::multipart::MultipartError> for RouteError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        Self::Multipart(value.to_string())
    }
}

impl From<sea_query::error::Error> for RouteError {
    fn from(value: sea_query::error::Error) -> Self {
        Self::SeaQuery(value.to_string())
    }
}

impl From<sqlx::error::Error> for RouteError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::Sqlx(value.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for RouteError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::JWT(value.to_string())
    }
}
