use crate::{
    model::{
        schema::Schema,
        schemas::user_management::users::{SunUsers, SunUsersIden},
    },
    route::error::RouteResult,
};
use argon2::{password_hash::rand_core::OsRng, Argon2, PasswordHash, PasswordVerifier};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Pool, Postgres};

pub async fn create_user(
    email: &str,
    password: &str,
    pool: &Pool<Postgres>,
) -> RouteResult<SunUsers> {
    let argon2 = Argon2::default();
    let salt = argon2::password_hash::SaltString::generate(&mut OsRng);
    let mut hash = vec![0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt.as_str().as_bytes(), &mut hash)?;

    let (sql, values) = Query::insert()
        .into_table((Schema::UserManagement, SunUsersIden::Table))
        .columns([SunUsersIden::Email, SunUsersIden::Password])
        .values([Expr::val(email).into(), Expr::val(hash).into()])?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<Postgres, SunUsers, _>(&sql, values)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_email(
    email: &str,
    pool: &Pool<Postgres>,
) -> RouteResult<Option<SunUsers>> {
    let (sql, values) = Query::select()
        .from((Schema::UserManagement, SunUsersIden::Table))
        .columns([
            SunUsersIden::Id,
            SunUsersIden::Email,
            SunUsersIden::Password,
            SunUsersIden::CreatedAt,
        ])
        .and_where(
            Expr::col((
                Schema::UserManagement,
                SunUsersIden::Table,
                SunUsersIden::Email,
            ))
            .eq(email),
        )
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<Postgres, SunUsers, _>(&sql, values)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub fn verify_user(user: &SunUsers, password: &str) -> RouteResult<()> {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&user.password);
    println!("{:?}", hash);
    let r = argon2.verify_password(password.as_bytes(), &hash?);
    println!("{:?}", r);
    r?;
    Ok(())
}
