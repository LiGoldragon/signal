#![cfg(feature = "transport")]

use signal::{AsyncFrameReading, AsyncFrameWriting, FrameBody, FrameCapacity, FrameError};
use tokio::io::duplex;

#[tokio::test]
async fn an_asynchronously_written_frame_reads_back_as_the_same_body() {
    let (mut writer, mut reader) = duplex(1024);
    let body = FrameBody::from(b"lojix deploy submission".to_vec());
    writer
        .write_frame(&body, FrameCapacity::default())
        .await
        .expect("write");
    let read = reader
        .read_frame(FrameCapacity::default())
        .await
        .expect("read");
    assert_eq!(read, body);
}

#[tokio::test]
async fn an_asynchronous_body_past_capacity_is_refused() {
    let (mut writer, _reader) = duplex(1024);
    let body = FrameBody::from(vec![0; 9]);
    let refusal = writer
        .write_frame(&body, FrameCapacity::from(8))
        .await
        .expect_err("refusal");
    assert!(matches!(
        refusal,
        FrameError::BodyTooLarge {
            found: 9,
            capacity: 8
        }
    ));
}
