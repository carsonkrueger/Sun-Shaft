use super::path::ROOT_ABSOLUTE_PATH;
use std::path::PathBuf;

const MEDIA_PATH: &'static str = "/media";

pub fn collection_path(collection_id: i32, media_id: i32) -> PathBuf {
    let media_path = PathBuf::from(format!(
        "/{}/{}/{}.mp4",
        MEDIA_PATH, collection_id, media_id
    ));
    ROOT_ABSOLUTE_PATH.join(media_path)
}

pub fn item_path(media_id: i32) -> PathBuf {
    let media_path = PathBuf::from(format!("/{}/{}.mp4", MEDIA_PATH, media_id));
    ROOT_ABSOLUTE_PATH.join(media_path)
}
