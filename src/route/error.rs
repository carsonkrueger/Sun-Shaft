use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use steamworks::AppId;

pub type RouteResult<T> = Result<T, RouteError>;

pub enum RouteError {
    SAInitError(steamworks::SteamAPIInitError),
    SAAppNotInstalled(AppId),
    FFMpeg(ffmpeg_next::Error),
    IO(std::io::Error),
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        match self {
            RouteError::SAInitError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::IO(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            RouteError::SAAppNotInstalled(id) => (
                StatusCode::BAD_REQUEST,
                format!("App {} not installed", id.0),
            )
                .into_response(),
            Self::FFMpeg(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
}

impl From<steamworks::SteamAPIInitError> for RouteError {
    fn from(value: steamworks::SteamAPIInitError) -> Self {
        Self::SAInitError(value)
    }
}

impl From<ffmpeg_next::Error> for RouteError {
    fn from(value: ffmpeg_next::Error) -> Self {
        Self::FFMpeg(value)
    }
}

impl From<std::io::Error> for RouteError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}
