use std::process::{Child, Command};

pub fn swap_display(display_num: u32) {
    let display_str = format!(":{}", display_num);
    std::env::set_var("DISPLAY", display_str);
}

pub fn start_virtual_display(display_num: u32, resolution: &str) -> Result<Child, std::io::Error> {
    let display_str = format!(":{}", display_num);
    todo!()
    // Command::new("Xvfb")
    //     .arg(&display_str)
    //     .arg("-screen")
    //     .arg("0")
    //     .arg(resolution)
    //     .spawn()
}
