use std::io::Cursor;
use std::pin::Pin;

use futures::StreamExt;
use signal::{
    ByteViewable, ComponentKind, Following, Frame, FrameCapacity, FrameReading, FrameWriting,
    HostName, NetworkEndpoint, NetworkPort, Restorable, Signal, Signalizable, Stream,
};

/// A subscription to the components a peer reaches: its state is the roster
/// it holds, and each change an endpoint it learned.
type ReachFrame = Frame<Vec<ComponentKind>, NetworkEndpoint>;

fn frames() -> Vec<ReachFrame> {
    vec![
        Frame::Opened(vec![ComponentKind::Mirror, ComponentKind::Lojix]),
        Frame::Changed(NetworkEndpoint {
            host_name: HostName("mirror.test".into()),
            network_port: NetworkPort(443),
        }),
        Frame::Lagged(vec![ComponentKind::Orchestrate]),
    ]
}

#[test]
fn every_frame_round_trips_through_the_archive() {
    for frame in frames() {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&frame).expect("archive a frame");
        let restored =
            rkyv::from_bytes::<ReachFrame, rkyv::rancor::Error>(&bytes).expect("restore a frame");
        assert_eq!(restored, frame);
    }
}

#[test]
fn every_frame_round_trips_through_the_wire_framing() {
    let mut wire = Vec::new();
    for frame in frames() {
        wire.write_frame(
            &frame.signalize().expect("signalize"),
            FrameCapacity::default(),
        )
        .expect("write");
    }
    let mut reader = Cursor::new(wire);
    for frame in frames() {
        let body = reader.read_frame(FrameCapacity::default()).expect("read");
        let restored: ReachFrame = Signal::<ReachFrame>::from(body.bytes().to_vec())
            .restore()
            .expect("restore");
        assert_eq!(restored, frame);
    }
}

#[test]
fn a_stream_yields_its_frames_in_order() {
    let source: Pin<Box<dyn futures_core::Stream<Item = ReachFrame> + Send>> =
        Box::pin(futures::stream::iter(frames()));
    let stream: Stream<Vec<ComponentKind>, NetworkEndpoint> = Stream::from(source);
    let yielded: Vec<ReachFrame> = futures::executor::block_on(stream.collect());
    assert_eq!(yielded, frames());
}

/// What a subscriber to the reached endpoints holds: each endpoint it has
/// learned, in the order it learned them.
#[derive(Clone, Debug, PartialEq)]
struct Reach(Vec<NetworkEndpoint>);

impl Following for Reach {
    type Change = NetworkEndpoint;

    fn after(mut state: Self, change: NetworkEndpoint) -> Self {
        state.0.push(change);
        state
    }
}

#[test]
fn a_subscriber_follows_the_frames_by_the_state_fold() {
    let endpoint = |port| NetworkEndpoint {
        host_name: HostName("mirror.test".into()),
        network_port: NetworkPort(port),
    };
    let frames: Vec<Frame<Reach, NetworkEndpoint>> = vec![
        Frame::Opened(Reach(vec![])),
        Frame::Changed(endpoint(1)),
        Frame::Changed(endpoint(2)),
        Frame::Lagged(Reach(vec![endpoint(3)])),
        Frame::Changed(endpoint(4)),
    ];
    let held = frames.into_iter().fold(None, |held, frame| match frame {
        Frame::Opened(state) | Frame::Lagged(state) => Some(state),
        Frame::Changed(change) => held.map(|state| Reach::after(state, change)),
    });
    assert_eq!(held, Some(Reach(vec![endpoint(3), endpoint(4)])));
}

#[cfg(feature = "datom")]
#[test]
fn every_frame_round_trips_through_datom() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let heads = ["Opened.[", "Changed.{", "Lagged.["];
    for (frame, head) in frames().into_iter().zip(heads) {
        let text = frame.datomize(vec![]).protosize().textualize();
        assert!(text.starts_with(head), "{text} opens with {head}");
        let restored = Potential::<ReachFrame>::from(text)
            .actualize(&mut Budget {
                remaining: 1024,
                reader: ReaderBudget::try_from(1024).unwrap(),
                depth: 0,
                maximum_depth: 1024,
            })
            .expect("actualize a frame");
        assert_eq!(restored, frame);
    }
}
