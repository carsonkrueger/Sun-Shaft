mod game;
mod hello_world;

use axum::Router;
use game::GameRoute;
use hello_world::HelloWorldRoute;
use sqlx::{Pool, Postgres};

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

const PUBLIC_ROUTES: &[&dyn PublicRoute] = &[&HelloWorldRoute, &GameRoute];

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
