use crate::{
    route::{error::RouteResult, state::AppState, PublicRoute, RoutePath},
    services::{self, media::chunk_name, response::buffer_to_stream_response},
};
use axum::{body::Body, extract::Path, response::Response, routing::get, Router};

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
) -> RouteResult<Response<Body>> {
    let dir = services::media::collection_dir(collection_id, media_id);
    let path = dir.join(chunk_name(offset, "mp4"));
    let mut buffer = Vec::new();
    services::media::get_media_chunk(&path, &mut buffer).await?;

    let buffer_len = buffer.len();
    let response = buffer_to_stream_response(buffer, buffer_len, "video/mp4").await;

    Ok(response)
}
