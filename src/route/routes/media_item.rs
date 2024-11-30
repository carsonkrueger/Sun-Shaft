use super::super::{AppState, PublicRoute, RoutePath};
use crate::{route::error::RouteResult, services};
use axum::{extract::Path, routing::get, Router};

pub struct MediaItemRoute;

impl RoutePath for MediaItemRoute {
    fn path(&self) -> &'static str {
        &"/media_item"
    }
}

impl PublicRoute for MediaItemRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/:media_id/:offset", get(get_item_chunk))
    }
}

async fn get_item_chunk(Path((media_id, offset)): Path<(i32, u32)>) -> RouteResult<()> {
    let path = services::media::item_path(media_id);
    let duration_seconds = 10;
    let mut buffer = Vec::new();
    services::media::get_media_chunk(path, offset, duration_seconds, &mut buffer)?;
    println!("{:?}", buffer);

    Ok(())
}
