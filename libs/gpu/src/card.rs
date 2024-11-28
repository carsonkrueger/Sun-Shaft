use std::{
    fs::{File, OpenOptions},
    os::fd::{AsFd, AsRawFd},
    path::Path,
};

pub struct Card(File);

impl Card {
    pub fn open(path: &Path) -> Self {
        let mut options = OpenOptions::new();
        options.read(true);
        //options.write(true);
        Card(options.open(path).unwrap())
    }
}

impl AsFd for Card {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for Card {
    fn as_raw_fd(&self) -> std::os::unix::prelude::RawFd {
        self.0.as_raw_fd()
    }
}

impl drm::Device for Card {}
impl drm::control::Device for Card {}
