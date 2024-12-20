use crate::model::error::ModelError;
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
    RouteMissingPermission,
    PermissionUnathorized,
    Argon2(String),
    Argon2PasswordHash(String),
    Model(String),
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
            RouteError::RouteMissingPermission => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Route missing permission".to_string(),
            ),
            PermissionUnauthorized => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "User unauthorized".to_string(),
            ),
            Self::Argon2(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            Self::Argon2PasswordHash(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            Self::Model(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
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

impl From<argon2::Error> for RouteError {
    fn from(value: argon2::Error) -> Self {
        Self::Argon2(value.to_string())
    }
}

impl From<argon2::password_hash::Error> for RouteError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Argon2PasswordHash(value.to_string())
    }
}

impl From<ModelError> for RouteError {
    fn from(value: ModelError) -> Self {
        Self::Model(value.to_string())
    }
}
