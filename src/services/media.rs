use super::path::ROOT_ABSOLUTE_PATH;
use std::{ffi::OsStr, path::PathBuf, process::Stdio};
use tokio::process::Command;

const MEDIA_PATH: &'static str = "media";

pub fn collection_path(collection_id: i32, media_id: i32, extension: &str) -> PathBuf {
    let media_path = PathBuf::from(format!(
        "/{}/{}/{}{}",
        MEDIA_PATH, collection_id, media_id, extension
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

pub async fn get_media_chunk<T: AsRef<OsStr>>(
    input_path: T,
    start: u32,
    duration: u32,
    output_buffer: &mut Vec<u8>,
) -> tokio::io::Result<()> {
    let ffmpeg_process = Command::new("ffmpeg")
        // .arg("-nostats")
        .arg("-loglevel")
        .arg("quiet")
        .arg("-hwaccel")
        .arg("vaapi")
        .arg("-vaapi_device")
        .arg("/dev/dri/renderD128")
        .arg("-i")
        .arg(input_path)
        .arg("-ss")
        .arg(start.to_string())
        .arg("-t")
        .arg(duration.to_string())
        .arg("-vf")
        .arg("format=nv12,hwupload")
        .arg("-c:v")
        // .arg("libx264")
        .arg("h264_vaapi")
        .arg("-movflags")
        .arg("frag_keyframe+empty_moov")
        .arg("-f")
        .arg("mp4")
        // .arg("-")
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .spawn()?;

    let out = ffmpeg_process.wait_with_output().await?;

    output_buffer.resize(out.stdout.len(), 0);
    output_buffer.copy_from_slice(&out.stdout);
    println!("Captured {} bytes of MP4 data", output_buffer.len());

    Ok(())
}
