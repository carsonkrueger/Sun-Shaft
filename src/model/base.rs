use super::{error::ModelResult, schema::IntoSchemaTableRef};
use sea_query::{IntoIden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow, Pool, Postgres};

pub async fn insert_returning<'r, M, C, CI, VI>(
    columns: CI,
    values: VI,
    pool: &Pool<Postgres>,
) -> ModelResult<M>
where
    M: Send + Unpin + for<'fr> FromRow<'fr, PgRow> + IntoSchemaTableRef,
    C: IntoIden,
    CI: IntoIterator<Item = C>,
    VI: IntoIterator<Item = SimpleExpr>,
{
    let (sql, values) = Query::insert()
        .into_table(M::schema_table_ref())
        .columns::<C, CI>(columns.into())
        .values::<VI>(values)?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let model = sqlx::query_as_with::<Postgres, M, _>(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(model)
}
