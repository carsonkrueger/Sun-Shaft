use sea_query::{Iden, TableRef};

pub trait IntoSchemaTableRef {
    fn schema_table_ref() -> TableRef;
}

#[derive(Clone)]
pub enum Schema {
    MediaManagement,
    UserManagement,
}

impl Iden for Schema {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        let name = match self {
            Self::MediaManagement => "media_management",
            Self::UserManagement => "user_management",
        };
        write!(s, "{}", &name).expect("Iden unquoted - Schema");
    }
}
