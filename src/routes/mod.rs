mod hello_world;
mod media;
mod media_row;
mod steam;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Router,
};
use hello_world::HelloWorldRoute;
use media::MediaRoute;
use media_row::MediaRowRoute;
use sqlx::{Pool, Postgres};
use steam::SteamRoute;
use steamworks::AppId;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

pub trait RoutePath {
    fn path(&self) -> &'static str;
}

pub trait PublicRoute: RoutePath {
    fn router(&self) -> Router<AppState>;
}

const PUBLIC_ROUTES: &[&dyn PublicRoute] =
    &[&HelloWorldRoute, &SteamRoute, &MediaRoute, &MediaRowRoute];
// const PRIVATE_ROUTES: [NestedRoute; 1] = [];

pub fn create_routes(state: AppState) -> Router {
    // Router::new()
    //         .nest(HelloWorldRoute::PATH, HelloWorldRoute::router())
    //         .nest(UserRoute::PATH, UserRoute::router())
    //         .nest(ContentRoute::PATH, ContentRoute::router())
    //         .layer(from_fn(validate_auth))
    //         .nest(ExercisePresetRoute::PATH, ExercisePresetRoute::router())
    //         .nest(AuthRoute::PATH, AuthRoute::router())
    //         .layer(from_fn(ctx_resolver))
    //         .layer(map_response(logger))
    //         .layer(CookieManagerLayer::new())
    //         .with_state(app_state)
    let mut router = Router::new();
    for &r in PUBLIC_ROUTES {
        router = router.nest(r.path(), r.router());
    }
    router.with_state(state)
}

pub type RouteResult<T> = Result<T, RouteError>;

pub enum RouteError {
    SAInitError(steamworks::SteamAPIInitError),
    SAAppNotInstalled(AppId),
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        match self {
            RouteError::SAInitError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            RouteError::SAAppNotInstalled(id) => (
                StatusCode::BAD_REQUEST,
                format!("App {} not installed", id.0),
            )
                .into_response(),
        }
    }
}

impl From<steamworks::SteamAPIInitError> for RouteError {
    fn from(value: steamworks::SteamAPIInitError) -> Self {
        Self::SAInitError(value)
    }
}
