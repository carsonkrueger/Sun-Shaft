use std::io::Cursor;

use axum::{
    body::Body,
    http::{header, Response},
};
use tokio_util::io::ReaderStream;

pub async fn buffer_to_stream_response<T: Unpin + AsRef<[u8]> + Send + 'static>(
    buffer: T,
    buffer_len: usize,
    content_type: &str,
) -> Response<Body> {
    let body = Body::from_stream(ReaderStream::new(Cursor::new(buffer)));
    let response = Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, buffer_len)
        .body(body)
        .unwrap();
    response
}
