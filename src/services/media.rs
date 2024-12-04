use crate::route::error::RouteResult;

use super::path::ROOT_ABSOLUTE_PATH;
use std::{
    ffi::OsStr,
    io::Write,
    path::{Path, PathBuf},
};
use tokio::{fs::File, io::AsyncReadExt, process::Command};

const MEDIA_PATH: &'static str = "media";

pub fn collection_dir(collection_id: i32, media_id: i32) -> PathBuf {
    let media_path = PathBuf::from(format!("/{}/{}/{}", MEDIA_PATH, collection_id, media_id));
    ROOT_ABSOLUTE_PATH.join(media_path)
}

pub fn item_dir(media_id: i64) -> PathBuf {
    let media_path = PathBuf::from(format!("./{}/{}", MEDIA_PATH, media_id));
    ROOT_ABSOLUTE_PATH.join(media_path)
}

pub fn chunk_name(chunk_index: u32, ext: &str) -> PathBuf {
    format!("{:05}.{}", chunk_index, ext).into()
}

pub async fn get_media_chunk(path: &Path, output_buffer: &mut Vec<u8>) -> RouteResult<()> {
    let mut file = File::options().read(true).open(path).await?;
    file.read_to_end(output_buffer).await?;
    Ok(())
}

pub async fn chop_and_store<T: AsRef<OsStr>>(
    input_file: T,
    output_dir: &Path,
) -> tokio::io::Result<()> {
    let file_pattern = "%05d.mp4";
    let out_dir_path = output_dir.join(file_pattern);
    let mut ffmpeg_process = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file)
        .arg("-c")
        .arg("copy")
        .arg("-map")
        .arg("0")
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg("8")
        .arg("-reset_timestamps")
        .arg("1")
        .arg(out_dir_path)
        .spawn()?;
    ffmpeg_process.wait().await?;
    Ok(())
}
