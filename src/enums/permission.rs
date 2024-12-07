use enum_derive::IntoEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Permission {
    Item(Item),
    PermissionLevels(PermissionLevels),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, IntoEnum)]
#[into_enum(Permission)]
pub enum Item {
    GetItem,
    PostItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, IntoEnum)]
#[into_enum(Permission)]
pub enum PermissionLevels {
    GetPermissionLevel,
    PostPermissionLevel,
}
