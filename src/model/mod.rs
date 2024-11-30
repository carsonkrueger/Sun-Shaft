pub mod base;
pub mod error;

pub trait DbModel {
    const TABLE: &'static str;
}
