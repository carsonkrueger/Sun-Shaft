use sea_query::{enum_def, IntoIden, TableRef};
use sqlx::{types::chrono, FromRow};

use crate::model::schema::{IntoSchemaTableRef, Schema};

#[derive(FromRow, Debug)]
#[enum_def]
pub struct SunUsers {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

impl IntoSchemaTableRef for SunUsers {
    fn schema_table_ref() -> sea_query::TableRef {
        TableRef::SchemaTable(
            Schema::UserManagement.into_iden(),
            SunUsersIden::Table.into_iden(),
        )
    }
}
