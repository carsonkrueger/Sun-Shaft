use super::path::ROOT_ABSOLUTE_PATH;
use std::{
    ffi::OsStr,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

const MEDIA_PATH: &'static str = "media";

pub fn collection_path(collection_id: i32, media_id: i32) -> PathBuf {
    let media_path = PathBuf::from(format!(
        "/{}/{}/{}.mp4",
        MEDIA_PATH, collection_id, media_id
    ));
    ROOT_ABSOLUTE_PATH.join(media_path)
}

pub fn item_path(media_id: i64, extension: &str) -> PathBuf {
    let media_path = PathBuf::from(format!(
        "./{}/{}/{}{}",
        MEDIA_PATH, media_id, media_id, extension
    ));
    ROOT_ABSOLUTE_PATH.join(media_path)
}

pub fn get_media_chunk<T: AsRef<OsStr>>(
    input_path: T,
    start: u32,
    duration: u32,
    output_buffer: &mut Vec<u8>,
) -> std::io::Result<()> {
    let mut ffmpeg_process = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-ss")
        .arg(start.to_string())
        .arg("-c:v")
        .arg("-f")
        .arg("mp4")
        .arg("-t")
        .arg(duration.to_string())
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()?;

    // Capture the stdout stream
    if let Some(mut stdout) = ffmpeg_process.stdout.take() {
        stdout.read_to_end(output_buffer)?;
        println!("Captured {} bytes of MP4 data", output_buffer.len());
    }

    ffmpeg_process.wait()?;
    Ok(())
}
