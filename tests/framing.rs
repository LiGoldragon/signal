use std::io::Cursor;

use signal::{
    ByteViewable, Capacious, Framable, FrameBody, FrameCapacity, FrameError, FrameReading,
    FrameWriting, LengthDeclaring, MAXIMUM_SIGNAL_BYTES, Restorable, Signal, Signalizable,
    StandardSocket,
};

#[test]
fn a_framed_body_carries_the_hand_computed_big_endian_prefix() {
    let body = FrameBody::from(vec![0xAA, 0xBB, 0xCC]);
    let framed = body.framed(FrameCapacity::default()).expect("frame");
    assert_eq!(framed, vec![0x00, 0x00, 0x00, 0x03, 0xAA, 0xBB, 0xCC]);
}

#[test]
fn a_written_frame_reads_back_as_the_same_body() {
    let body = FrameBody::from(b"orchestrate lock request".to_vec());
    let mut wire = Vec::new();
    wire.write_frame(&body, FrameCapacity::default())
        .expect("write");
    let read = Cursor::new(wire)
        .read_frame(FrameCapacity::default())
        .expect("read");
    assert_eq!(read, body);
}

#[test]
fn a_body_past_capacity_is_refused_rather_than_written() {
    let body = FrameBody::from(vec![0; 9]);
    let refusal = body.framed(FrameCapacity::from(8)).expect_err("refusal");
    assert!(matches!(
        refusal,
        FrameError::BodyTooLarge {
            found: 9,
            capacity: 8
        }
    ));
}

#[test]
fn a_declared_length_past_capacity_is_refused_before_the_body_is_allocated() {
    let mut wire = vec![0xFF, 0xFF, 0xFF, 0xFF];
    wire.extend_from_slice(b"short");
    let refusal = Cursor::new(wire)
        .read_frame(FrameCapacity::default())
        .expect_err("refusal");
    assert!(matches!(
        refusal,
        FrameError::BodyTooLarge {
            found: 4_294_967_295,
            capacity: MAXIMUM_SIGNAL_BYTES
        }
    ));
}

#[test]
fn the_default_capacity_admits_exactly_eight_mebibytes() {
    let capacity = FrameCapacity::default();
    assert_eq!(capacity.capacity_bytes(), 8 * 1024 * 1024);
    assert!(capacity.admits(8 * 1024 * 1024));
    assert!(!capacity.admits(8 * 1024 * 1024 + 1));
}

#[test]
fn a_prefix_declares_the_length_its_bytes_spell() {
    assert_eq!(
        signal::FramePrefix::from([0x00, 0x01, 0x00, 0x00]).declared_length(),
        65_536
    );
}

#[test]
fn a_signalized_value_frames_and_restores_across_the_wire() {
    let socket = StandardSocket::UnixSocket("/run/orchestrate/ordinary.sock".into());
    let signalized = socket.signalize().expect("signalize");
    let mut wire = Vec::new();
    wire.write_frame(&signalized, FrameCapacity::default())
        .expect("write");
    let body = Cursor::new(wire)
        .read_frame(FrameCapacity::default())
        .expect("read");
    let restored: StandardSocket = Signal::<StandardSocket>::from(body.bytes().to_vec())
        .restore()
        .expect("restore");
    assert_eq!(restored, socket);
}
