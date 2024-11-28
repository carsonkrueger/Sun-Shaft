use super::{AppState, PublicRoute, RoutePath, RouteResult};
use axum::{extract::Path, routing::get, Router};

pub struct MediaRoute;

impl RoutePath for MediaRoute {
    fn path(&self) -> &'static str {
        &"/media"
    }
}

impl PublicRoute for MediaRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/:media_id/:offset", get(get_media_chunk))
    }
}

async fn get_media_chunk(Path((media_id, offset)): Path<(u32, u32)>) -> RouteResult<()> {
    Ok(())
}
