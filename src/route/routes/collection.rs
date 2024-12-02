use std::pin::pin;

use super::super::{AppState, PublicRoute, RoutePath};
use crate::{
    route::error::RouteResult,
    services::{self, response::buffer_to_stream_response},
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
    let path = services::media::collection_path(collection_id, media_id, ".mp4");
    let duration_seconds = 10;
    let mut buffer = Vec::new();
    services::media::get_media_chunk(path, offset, duration_seconds, &mut buffer).await?;

    let buffer_len = buffer.len();
    let response = buffer_to_stream_response(buffer, buffer_len, "video/mp4").await;

    Ok(response)
}
