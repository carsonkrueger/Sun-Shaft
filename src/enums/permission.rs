use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Item(Item),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    GetItem,
}
