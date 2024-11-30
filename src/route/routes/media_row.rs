use crate::route::error::RouteResult;

use super::super::{AppState, PublicRoute, RoutePath};
use axum::{extract::Path, routing::get, Router};

pub struct MediaRowRoute;

impl RoutePath for MediaRowRoute {
    fn path(&self) -> &'static str {
        &"/row"
    }
}

impl PublicRoute for MediaRowRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/:cat_id/:offset", get(get_row))
    }
}

async fn get_row(Path((cat, off)): Path<(u32, u32)>) -> RouteResult<()> {
    Ok(())
}
