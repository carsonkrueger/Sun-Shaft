use std::io::Cursor;

use super::{AppState, PublicRoute, RoutePath, RouteResult};
use axum::{extract::Path, routing::get, Router};
use ffmpeg_next::{codec::traits::Decoder, Frame, Packet};
use tokio::sync::mpsc::unbounded_channel;

pub struct MediaRoute;

impl RoutePath for MediaRoute {
    fn path(&self) -> &'static str {
        &"/media"
    }
}

impl PublicRoute for MediaRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new().route("/:collection_id/:media_id/:offset", get(get_media_chunk))
    }
}

async fn get_media_chunk(
    Path((collection_id, media_id, offset)): Path<(i32, i32, i32)>,
) -> RouteResult<()> {
    // use get media chunk service

    Ok(())
}
