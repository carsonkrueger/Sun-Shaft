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
        Router::new().route("/:collection_id/:media_id/:offset", get(get_media_chunk))
    }
}

async fn get_media_chunk(
    Path((collection_id, media_id, offset)): Path<(i32, i32, i32)>,
) -> RouteResult<()> {
    ffmpeg_next::init()?;

    let media_path = crate::services::media::collection_path(collection_id, media_id);
    let mut ictx = ffmpeg_next::format::input(&media_path)?;
    let streams = ictx.streams().best(ffmpeg_next::media::Type::Video)?;
    let stream_index = streams.index();

    ictx.seek(
        offset as i64 * ffmpeg_next::ffi::AV_TIME_BASE as i64,
        todo!(),
    )?;

    Ok(())
}
