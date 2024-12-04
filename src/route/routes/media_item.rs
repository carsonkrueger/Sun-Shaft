use super::super::{AppState, PublicRoute, RoutePath};
use crate::model::schema::Schema;
use crate::model::schemas::media_management::items::ItemsIden;
use crate::services::media::{chunk_name, get_media_chunk};
use crate::services::response::buffer_to_stream_response;
use crate::{route::error::RouteResult, services};
use axum::body::{Body, Bytes};
use axum::extract::{DefaultBodyLimit, State};
use axum::response::Response;
use axum::routing::post;
use axum::{extract::Path, routing::get, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::Postgres;
use tokio::io::AsyncWriteExt;

pub struct MediaItemRoute;

impl RoutePath for MediaItemRoute {
    fn path(&self) -> &'static str {
        &"/media_item"
    }
}

impl PublicRoute for MediaItemRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new()
            .route("/:media_id/:offset", get(get_item_chunk))
            .route("/", post(post_item))
            .layer(DefaultBodyLimit::max(250 * 1024 * 1024)) // 250 GB limit
    }
}

async fn get_item_chunk(Path((media_id, offset)): Path<(i64, u32)>) -> RouteResult<Response<Body>> {
    let dir = services::media::item_dir(media_id);
    let path = dir.join(chunk_name(offset, "mp4"));
    let mut output_buffer = Vec::new();
    get_media_chunk(&path, &mut output_buffer).await?;
    let buffer_len = output_buffer.len();
    let response = buffer_to_stream_response(output_buffer, buffer_len, "video/mp4").await;

    Ok(response)
}

#[derive(TryFromMultipart)]
struct PostItemBody {
    pub title: String,
    pub description: String,
    #[form_data(limit = "unlimited")]
    pub file: FieldData<Bytes>,
}

async fn post_item(
    State(s): State<AppState>,
    TypedMultipart(body): TypedMultipart<PostItemBody>,
) -> RouteResult<()> {
    let (sql, values) = Query::insert()
        .into_table((Schema::MediaManagement, ItemsIden::Table))
        .columns([ItemsIden::Title, ItemsIden::Description])
        .values([body.title.into(), body.description.into()])?
        .returning_col(ItemsIden::Id)
        .build_sqlx(PostgresQueryBuilder);

    let (item_id,) = sqlx::query_as_with::<Postgres, (i64,), _>(&sql, values)
        .fetch_one(&s.pool)
        .await?;

    let input_path = format!("/home/carson/Repos/sun-shaft/{}.mp4", item_id);
    let mut f = async_tempfile::TempFile::new_with_name(input_path.clone())
        .await
        .unwrap();
    f.open_rw()
        .await
        .unwrap()
        .write_all(&body.file.contents)
        .await
        .unwrap();
    f.flush().await.unwrap();

    let output_dir = services::media::item_dir(item_id);
    std::fs::create_dir_all(output_dir.clone())?;
    services::media::chop_and_store::<String>(input_path.into(), &output_dir).await?;

    Ok(())
}
