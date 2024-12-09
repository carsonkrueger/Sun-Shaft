use futures::StreamExt;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{prelude::FromRow, Pool, Postgres};

use crate::model::{
    error::ModelResult,
    schema::Schema,
    schemas::user_management::permissions::{
        PermissionLevelsIden, PermissionsIden, PermissionsPermissionLevelsIden,
    },
};

#[derive(FromRow, Debug)]
struct UserPermissionJoin {
    pub id: i32,
    pub permission_id: i32,
    pub permission_level_id: i32,
    pub permission_name: String,
    pub permission_level_name: String,
}

pub async fn get_user_permissions(user_id: i64, pool: &Pool<Postgres>) -> ModelResult<()> {
    let schema = Schema::UserManagement;
    let (sql, _) = Query::select()
        .from((schema.clone(), PermissionsPermissionLevelsIden::Table))
        .columns([(
            schema.clone(),
            PermissionsPermissionLevelsIden::Table,
            PermissionsPermissionLevelsIden::Id,
        )])
        .columns([
            (schema.clone(), PermissionsIden::Table, PermissionsIden::Id),
            (
                schema.clone(),
                PermissionsIden::Table,
                PermissionsIden::Name,
            ),
        ])
        .columns([
            (
                schema.clone(),
                PermissionLevelsIden::Table,
                PermissionLevelsIden::Id,
            ),
            (
                schema.clone(),
                PermissionLevelsIden::Table,
                PermissionLevelsIden::Name,
            ),
        ])
        .join(
            sea_query::JoinType::RightJoin,
            (schema.clone(), PermissionsIden::Table),
            Expr::col((
                schema.clone(),
                PermissionsPermissionLevelsIden::Table,
                PermissionsPermissionLevelsIden::PermissionId,
            ))
            .equals((schema.clone(), PermissionsIden::Table, PermissionsIden::Id)),
        )
        .join(
            sea_query::JoinType::RightJoin,
            (schema.clone(), PermissionLevelsIden::Table),
            Expr::col((
                schema.clone(),
                PermissionsPermissionLevelsIden::Table,
                PermissionsPermissionLevelsIden::PermissionId,
            ))
            .equals((
                schema.clone(),
                PermissionLevelsIden::Table,
                PermissionLevelsIden::Id,
            )),
        )
        .and_where(
            Expr::col((
                schema.clone(),
                PermissionsPermissionLevelsIden::Table,
                PermissionsPermissionLevelsIden::UserId,
            ))
            .eq(user_id),
        )
        .build_sqlx(PostgresQueryBuilder);

    let join_res = sqlx::query_as::<Postgres, UserPermissionJoin>(&sql)
        .fetch_all(pool)
        .await?;
    println!("{:?}", join_res);

    Ok(())
}
