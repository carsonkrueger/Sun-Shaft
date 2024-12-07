use crate::enums::permission::{Item, Permission};
use crate::middleware::permission::create_permission_mw;
use crate::model::schema::Schema;
use crate::model::schemas::media_management::items::ItemsIden;
use crate::route::state::AppState;
use crate::route::{RoutePath, RouteRouter};
use crate::services::media::{chunk_name, get_media_chunk};
use crate::services::path::ROOT_ABSOLUTE_PATH;
use crate::services::response::buffer_to_stream_response;
use crate::{route::error::RouteResult, services};
use axum::body::{Body, Bytes};
use axum::extract::{DefaultBodyLimit, State};
use axum::middleware::from_fn;
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

impl RouteRouter for MediaItemRoute {
    fn router(&self) -> axum::Router<AppState> {
        Router::new()
            .route(
                "/:media_id/:offset",
                get(get_item_chunk)
                    .route_layer(from_fn(create_permission_mw(Item::GetItem.into()))),
            )
            .route(
                "/",
                post(post_item).route_layer(from_fn(create_permission_mw(Item::PostItem.into()))),
            )
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

    let mut tx = s.pool.begin().await?;
    let (item_id,) = sqlx::query_as_with::<Postgres, (i64,), _>(&sql, values)
        .fetch_one(&mut *tx)
        .await?;

    // let extension = body.file.metadata.
    let input_path = ROOT_ABSOLUTE_PATH.join(format!("{}.mp4", item_id));
    let mut f = async_tempfile::TempFile::new_with_name(input_path.to_str().unwrap())
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
    services::media::chop_and_store(input_path, &output_dir).await?;

    tx.commit().await?;
    Ok(())
}
