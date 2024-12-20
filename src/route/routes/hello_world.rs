use axum::{response::IntoResponse, routing::get, Router};

use super::super::{AppState, RoutePath, RouteRouter};

pub struct HelloWorldRoute;

impl RoutePath for HelloWorldRoute {
    fn path(&self) -> &'static str {
        &"/hello-world"
    }
}

impl RouteRouter for HelloWorldRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/", get(hello_world))
    }
}

async fn hello_world() -> impl IntoResponse {
    "Hello World!!!1!"
}
