use sea_query::Iden;

pub enum Schema {
    MediaManagement,
}

impl Iden for Schema {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        let name = match self {
            Self::MediaManagement => "media_management",
        };
        write!(s, "{}", &name).expect("Iden unquoted - Schema");
    }
}
