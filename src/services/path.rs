use std::{cell::LazyCell, path::PathBuf};

pub const ROOT_ABSOLUTE_PATH: LazyCell<PathBuf> =
    LazyCell::new(|| std::env::current_dir().expect("Cur dir"));
