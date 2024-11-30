use super::super::{AppState, PublicRoute, RoutePath};
use crate::{route::error::RouteResult, services};
use axum::{extract::Path, routing::get, Router};

pub struct CollectionRoute;

impl RoutePath for CollectionRoute {
    fn path(&self) -> &'static str {
        &"/collection"
    }
}

impl PublicRoute for CollectionRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route(
            "/:collection_id/:media_id/:offset",
            get(get_collection_chunk),
        )
    }
}

async fn get_collection_chunk(
    Path((collection_id, media_id, offset)): Path<(i32, i32, u32)>,
) -> RouteResult<()> {
    let path = services::media::collection_path(collection_id, media_id);
    let duration_seconds = 10;
    let mut buffer = Vec::new();
    services::media::get_media_chunk(path, offset, duration_seconds, &mut buffer)?;
    println!("{:?}", buffer);

    Ok(())
}
