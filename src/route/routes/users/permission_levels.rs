use crate::{
    enums::permission::{Item, PermissionLevels},
    middleware::permission::create_permission_mw,
    route::{state::AppState, RoutePath, RouteRouter},
};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};

pub struct PermissionLevelsRoute;

impl RoutePath for PermissionLevelsRoute {
    fn path(&self) -> &'static str {
        &"/permissions_levels"
    }
}

impl RouteRouter for PermissionLevelsRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new()
            .route(
                "/user_id",
                get(get_permission_level).route_layer(from_fn(create_permission_mw(
                    PermissionLevels::GetPermissionLevel.into(),
                ))),
            )
            .route(
                "/",
                post(post_permission_level).route_layer(from_fn(create_permission_mw(
                    PermissionLevels::PostPermissionLevel.into(),
                ))),
            )
    }
}

async fn get_permission_level() {
    todo!()
}

async fn post_permission_level() {
    todo!()
}
