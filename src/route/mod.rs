pub mod error;
mod routes;
pub mod state;

use axum::Router;
use routes::hello_world::HelloWorldRoute;
use routes::media::collection::CollectionRoute;
use routes::media::item::MediaItemRoute;
use routes::media_row::MediaRowRoute;
use routes::steam::SteamRoute;
use state::AppState;

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
];
const PRIVATE_ROUTES: &[&dyn RouteRouter] = &[];

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
    for &r in PRIVATE_ROUTES {
        router = router.nest(r.path(), r.router());
    }
    // add auth MW here
    // router.layer() ...
    for &r in PUBLIC_ROUTES {
        router = router.nest(r.path(), r.router());
    }
    router.with_state(state)
}
