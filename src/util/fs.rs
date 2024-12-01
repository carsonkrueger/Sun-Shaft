use axum::body::Bytes;
use std::{io, path::Path};

pub async fn file_from_bytes(path: &Path, bytes: &Bytes) -> io::Result<()> {
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;
    tokio::fs::write(path, bytes).await?;
    Ok(())
}
