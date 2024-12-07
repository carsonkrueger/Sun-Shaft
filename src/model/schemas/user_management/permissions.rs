use sea_query::enum_def;
use sqlx::types::chrono;

#[enum_def]
pub struct Permissions {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDate,
}

#[enum_def]
pub struct PermissionLevels {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDate,
}

#[enum_def]
pub struct PermissionsPermissionLevels {
    pub id: i32,
    pub user_id: i32,
    pub permission_id: i32,
    pub permission_level_id: i32,
    pub created_at: chrono::NaiveDate,
}
