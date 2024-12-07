use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Postgres};

use crate::model::error::ModelResult;

pub async fn get_user_permissions(id: i64, pool: &Pool<Postgres>) -> ModelResult<()> {
    let (sql, values) = Query::select()
        // .columns(&[])
        .build_sqlx(PostgresQueryBuilder);

    let (item_id,) = sqlx::query_as_with::<Postgres, (i64,), _>(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(())
}
