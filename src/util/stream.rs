use axum::{body::Bytes, BoxError};
use futures::{Stream, TryStreamExt};
use std::{io, path::Path};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

pub async fn stream_to_file<S, E>(path: &Path, stream: S) -> Result<(), std::io::Error>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    // Convert the stream into an `AsyncRead`.
    let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    // Create the file. `File` implements `AsyncWrite`.
    let mut file = BufWriter::new(File::create(path).await?);

    // Copy the body into the file.
    tokio::io::copy(&mut body_reader, &mut file).await?;

    Ok(())
}
