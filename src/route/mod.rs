pub mod error;
mod routes;
pub mod state;

use crate::middleware::auth::ctx_resolver;
use axum::middleware::from_fn;
use axum::Router;
use routes::auth::AuthRoute;
use routes::hello_world::HelloWorldRoute;
use routes::media::collection::CollectionRoute;
use routes::media::item::MediaItemRoute;
use routes::media_row::MediaRowRoute;
use routes::steam::SteamRoute;
use state::AppState;
use tower_cookies::CookieManagerLayer;

pub trait RoutePath {
    fn path(&self) -> &'static str;
}

pub trait RouteRouter: RoutePath {
    fn router(&self) -> Router<AppState>;
}

const PUBLIC_ROUTES: &[&dyn RouteRouter] = &[
    &HelloWorldRoute,
    &SteamRoute,
    &CollectionRoute,
    &MediaItemRoute,
    &MediaRowRoute,
    &AuthRoute,
];
const PRIVATE_ROUTES: &[&dyn RouteRouter] = &[];

pub fn create_routes(state: AppState) -> Router {
    let mut router = Router::new();

    // PRIVATE ROUTES
    for &r in PRIVATE_ROUTES {
        router = router.nest(r.path(), r.router());
    }

    // AUTH MIDDLEWARE (protects private routes)
    router = router.layer(from_fn(ctx_resolver));

    // PUBLIC ROUTES
    for &r in PUBLIC_ROUTES {
        router = router.nest(r.path(), r.router());
    }

    // Logging, Cookies, State
    // .layer(map_response(logger))
    router = router.layer(CookieManagerLayer::new());
    router.with_state(state)
}
