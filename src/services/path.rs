use std::{cell::LazyCell, path::PathBuf};

pub const ROOT_ABSOLUTE_PATH: LazyCell<PathBuf> =
    LazyCell::new(|| find_project_root().expect("ROOT_ABSOLUTE_PATH"));

pub fn find_project_root() -> Option<PathBuf> {
    let mut cur_dir = std::env::current_dir().expect("Cur dir");
    while cur_dir.pop() {
        if cur_dir.ends_with("/sun-shaft") {
            return Some(cur_dir);
        }
    }
    None
}
