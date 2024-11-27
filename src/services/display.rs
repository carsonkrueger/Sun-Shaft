use std::fs::OpenOptions;
use std::os::fd::AsRawFd;

use glutin::api::egl::device::Device;
use glutin::{api::egl, config::Config, prelude::*};
use std::ops::Index;

pub fn swap_display(display_num: u32) {
    let display_str = format!(":{}", display_num);
    std::env::set_var("DISPLAY", display_str);
}

pub fn start_virtual_display(display_num: u32, w: i32, h: i32) -> glutin::error::Result<()> {
    let devices = egl::device::Device::query_devices()
        .expect("Query EGL devices")
        .take(1)
        .collect::<Vec<Device>>();
    let device_path = devices.index(0).drm_render_device_node_path().unwrap();
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(device_path)
        .expect("Open DRM device with R/W permissions");

    let drm_display_handle = raw_window_handle::DrmDisplayHandle::new(fd.as_raw_fd());
    let display_handle = raw_window_handle::RawDisplayHandle::Drm(drm_display_handle);

    let display = unsafe {
        glutin::display::Display::new(display_handle, glutin::display::DisplayApiPreference::Egl)?
    };
    let template = glutin::config::ConfigTemplateBuilder::new()
        .prefer_hardware_accelerated(Some(true))
        .build();
    let configs = unsafe {
        display
            .find_configs(template)?
            .take(1)
            .collect::<Vec<Config>>()
    };
    let config = configs.index(0);
    let drm_window_handle = raw_window_handle::DrmWindowHandle::new(display_num);
    let window_handle = raw_window_handle::RawWindowHandle::Drm(drm_window_handle);
    let context_attrs = glutin::context::ContextAttributesBuilder::new().build(Some(window_handle));
    let context = unsafe { display.create_context(config, &context_attrs)? };
    let surface = unsafe { display.create_window_surface(config, todo!())? };

    todo!();
}
