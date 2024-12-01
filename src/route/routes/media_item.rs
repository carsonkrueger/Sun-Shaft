use super::super::{AppState, PublicRoute, RoutePath};
use crate::model::schema::Schema;
use crate::model::schemas::media_management::items::ItemsIden;
use crate::util;
use crate::{route::error::RouteResult, services};
use axum::body::Bytes;
use axum::extract::{DefaultBodyLimit, State};
use axum::routing::post;
use axum::{extract::Path, routing::get, Router};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::Postgres;

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
            .layer(DefaultBodyLimit::max(250 * 1024 * 1024))
    }
}

async fn get_item_chunk(Path((media_id, offset)): Path<(i64, u32)>) -> RouteResult<()> {
    let path = services::media::item_path(media_id, ".mp4");
    let duration_seconds = 10;
    let mut buffer = Vec::new();
    services::media::get_media_chunk(path, offset, duration_seconds, &mut buffer)?;
    println!("buffer len: {}", buffer.len());

    Ok(())
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

    let path = services::media::item_path(item_id, ".mp4");
    util::fs::file_from_bytes(&path, &body.file.contents).await?;

    Ok(())
}
