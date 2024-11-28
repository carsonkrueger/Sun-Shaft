use crate::card::Card;
use drm::control::{framebuffer, Device};
use std::os::unix::io::AsRawFd;
use std::path::Path;

pub struct CardFacade(Card);

impl CardFacade {
    pub fn new(path: &Path) -> Self {
        Self(Card::open(path))
    }
    fn setup(&self) {
        let resources = self.0.resource_handles().unwrap();
        let connector = resources
            .connectors()
            .iter()
            .find(|&&conn| {
                self.0.get_connector(conn, false).unwrap().state()
                    == drm::control::connector::State::Connected
            })
            .expect("No connected connectors found");
    }
    pub fn capture_frame(&self, handle: framebuffer::Handle) -> Result<&[u8], ()> {
        let fb_info = self.0.get_framebuffer(handle).unwrap();
        let fd = self.0.as_raw_fd();
        let fb_size = (fb_info.pitch() * fb_info.depth() * 4) as usize;
        let fb_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                fb_size,
                libc::PROT_READ,
                libc::MAP_SHARED,
                fd,
                0,
            )
        };
        let framebuffer = unsafe { std::slice::from_raw_parts(fb_ptr as *const u8, fb_size) };
        Ok(framebuffer)
    }
}
