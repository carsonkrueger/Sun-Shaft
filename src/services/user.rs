use crate::{
    model::{
        schema::Schema,
        schemas::user_management::users::{Users, UsersIden},
    },
    route::error::RouteResult,
};
use argon2::password_hash::rand_core::OsRng;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Postgres};

pub async fn create_user(email: &str, password: &str, pool: &Pool<Postgres>) -> RouteResult<Users> {
    let params = argon2::Params::DEFAULT;
    let argon2 = argon2::Argon2::new(argon2::Algorithm::Argon2d, argon2::Version::V0x10, params);
    let salt = argon2::password_hash::SaltString::generate(&mut OsRng);
    let mut hash: Vec<u8> = Vec::new();
    argon2.hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut hash)?;

    let (sql, values) = Query::insert()
        .into_table((Schema::UserManagement, UsersIden::Table))
        .values([Expr::val(email).into(), Expr::val(hash).into()])?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<Postgres, Users, _>(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(user)
}
