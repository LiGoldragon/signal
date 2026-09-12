//! The exchange layer, exercised as a wire: values are framed, read back off
//! a byte stream and attributed to their exchange. Nothing here inspects
//! source text.

use std::io::Cursor;

use signal::{
    Answer, AuthorizedObjectInterest, AuthorizedObjectKind, ByteViewable, CONNECTION_EXCHANGE,
    ComponentClassification, ComponentKind, ComponentObjectInterest, Conclusion, ContractDigest,
    Contracted, Delivery, Differentiator, Dispatch, Ending, ExchangeFault, ExchangeId,
    ExchangeLedger, ExchangeMinting, ExchangeTracking, Exchanged, FrameCapacity, FrameReading,
    FrameWriting, Greeted, Handshake, HandshakeReceipt, HandshakeRejection, Opening, Restorable,
    Signal, Signalizable,
};

/// A stand-in contract. Its query and response roots are real generated
/// taxonomy enums, derived exactly as a contract's roots are, so what the
/// generic envelope is proved against is the shape it will actually carry.
type Query = AuthorizedObjectInterest;
type Response = ComponentClassification;

/// The authored source of the stand-in contract.
const PING_SOURCE: &str = "Signal\n[]\n[]\n[]\n[ Ping.Integer ]\n";

/// The same shape with a different body — a peer built from another source.
const PONG_SOURCE: &str = "Signal\n[]\n[]\n[]\n[ Pong.Integer ]\n";

/// FNV-1a over exactly [`PING_SOURCE`]'s bytes, taken as a signed 64-bit
/// integer, computed in Python from the published algorithm — an oracle
/// outside the crate under test.
const PING_DIGEST: ContractDigest = -501_540_396_992_439_027;

struct Ping;
struct Pong;

impl Contracted for Ping {
    const CONTRACT_SOURCE: &'static str = PING_SOURCE;
}

impl Contracted for Pong {
    const CONTRACT_SOURCE: &'static str = PONG_SOURCE;
}

fn greeted_ledger() -> ExchangeLedger {
    let mut ledger = ExchangeLedger::default();
    ledger.greet().expect("first greeting");
    ledger
}

fn query() -> Query {
    AuthorizedObjectInterest::ComponentObject(ComponentObjectInterest {
        component_kind: ComponentKind::Orchestrate,
        authorized_object_kind: AuthorizedObjectKind::Operation,
    })
}

fn response() -> Response {
    ComponentClassification {
        differentiator: Differentiator {
            component_kind: ComponentKind::Orchestrate,
            authorized_object_kind: AuthorizedObjectKind::Operation,
        },
        authorized_object_interest: AuthorizedObjectInterest::AnyAuthorizedObject,
    }
}

/// Put a value on a byte stream the way a socket carries it, and read it back.
fn across_the_wire<T>(value: &T) -> T
where
    T: Signalizable,
    Signal<T>: Restorable<T>,
{
    let capacity = FrameCapacity::default();
    let mut wire = Vec::new();
    wire.write_frame(&value.signalize().expect("signalize"), capacity)
        .expect("write");
    let body = Cursor::new(wire).read_frame(capacity).expect("read");
    Signal::<T>::from(body.bytes().to_vec())
        .restore()
        .expect("restore")
}

#[test]
fn a_contract_is_identified_by_the_digest_of_its_own_authored_source() {
    assert_eq!(Ping::contract_digest(), PING_DIGEST);
    assert_eq!(
        Ping::greeting(),
        Handshake {
            contract_digest: PING_DIGEST
        }
    );
}

#[test]
fn two_peers_built_from_the_same_source_settle_the_connection() {
    assert_eq!(
        Ping::receipt(&Ping::greeting()),
        HandshakeReceipt::Greeted(PING_DIGEST)
    );
}

#[test]
fn two_peers_built_from_different_sources_refuse_each_other() {
    assert_eq!(
        Ping::receipt(&Pong::greeting()),
        HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(PING_DIGEST))
    );
    assert_ne!(Pong::contract_digest(), Ping::contract_digest());
}

#[test]
fn two_exchanges_on_one_connection_are_named_apart() {
    let mut ledger = greeted_ledger();
    let first = ledger.open().expect("first exchange");
    let second = ledger.open().expect("second exchange");
    assert_ne!(first, second);
    assert!(ledger.bears(first));
    assert!(ledger.bears(second));
    assert_eq!(ledger.open_exchanges().len(), 2);
}

#[test]
fn no_minted_exchange_is_ever_the_connection_itself() {
    let mut ledger = greeted_ledger();
    for _ in 0..8 {
        assert_ne!(ledger.open().expect("exchange"), CONNECTION_EXCHANGE);
    }
}

#[test]
fn an_exchange_the_connection_does_not_hold_is_refused_as_unknown() {
    let ledger = greeted_ledger();
    assert_eq!(
        ledger.require_open(7),
        Err(ExchangeFault::UnknownExchange),
        "an unopened exchange is not borne"
    );
}

#[test]
fn reopening_a_live_exchange_is_refused_as_in_use() {
    let mut ledger = greeted_ledger();
    let exchange = ledger.open().expect("exchange");
    assert_eq!(
        ledger.admit(exchange),
        Err(ExchangeFault::ExchangeInUse),
        "an identifier already open cannot be reused"
    );
}

#[test]
fn an_exchange_opened_before_the_greeting_is_refused() {
    let mut ledger = ExchangeLedger::default();
    assert!(!ledger.is_greeted());
    assert_eq!(ledger.admit(1), Err(ExchangeFault::GreetingExpected));
}

#[test]
fn a_second_greeting_on_one_connection_is_refused() {
    let mut ledger = greeted_ledger();
    assert_eq!(ledger.greet(), Err(ExchangeFault::GreetingRepeated));
}

#[test]
fn the_open_set_is_bounded_and_refuses_past_its_capacity() {
    let mut ledger = ExchangeLedger::from(2);
    ledger.greet().expect("greeting");
    assert_eq!(ledger.capacity(), 2);
    ledger.open().expect("first");
    ledger.open().expect("second");
    assert_eq!(ledger.open(), Err(ExchangeFault::ExchangeLimit));
}

#[test]
fn an_abandoned_exchange_is_released_and_its_slot_returns() {
    let mut ledger = ExchangeLedger::from(1);
    ledger.greet().expect("greeting");
    let first = ledger.open().expect("first");
    assert_eq!(ledger.open(), Err(ExchangeFault::ExchangeLimit));
    assert_eq!(ledger.release(first), Ok(first));
    assert!(!ledger.bears(first));
    let second = ledger.open().expect("second after release");
    assert_ne!(second, first);
}

#[test]
fn releasing_an_exchange_twice_is_refused_as_unknown() {
    let mut ledger = greeted_ledger();
    let exchange = ledger.open().expect("exchange");
    assert_eq!(ledger.release(exchange), Ok(exchange));
    assert_eq!(
        ledger.release(exchange),
        Err(ExchangeFault::UnknownExchange)
    );
}

#[test]
fn a_greeting_crosses_the_wire_as_a_dispatch() {
    let sent: Dispatch<Query> = Dispatch::Greet(Ping::greeting());
    assert_eq!(across_the_wire(&sent), sent);
}

#[test]
fn a_query_crosses_the_wire_on_the_exchange_that_opened_it() {
    let sent: Dispatch<Query> = Dispatch::Open(Opening {
        exchange: 12,
        query: query(),
    });
    let received = across_the_wire(&sent);
    assert_eq!(received, sent);
    match received {
        Dispatch::Open(opening) => {
            assert_eq!(opening.exchange(), 12);
            assert_eq!(opening.query, query());
        }
        other => panic!("a query arrived as {other:?}"),
    }
}

#[test]
fn abandoning_an_exchange_crosses_the_wire_naming_only_it() {
    let sent: Dispatch<Query> = Dispatch::Abandon(4);
    assert_eq!(across_the_wire(&sent), sent);
}

/// The behaviour `signal-frame` could not express and every streaming
/// contract in the estate needs: one connection carrying a subscription that
/// goes on answering while a one-answer exchange opens, answers and ends
/// between its frames. Attribution is by exchange alone.
#[test]
fn a_subscription_and_a_one_answer_exchange_interleave_on_one_connection() {
    let capacity = FrameCapacity::default();
    let subscription: ExchangeId = 1;
    let one_answer: ExchangeId = 2;

    let sent: Vec<Delivery<Response>> = vec![
        Delivery::Greeted(HandshakeReceipt::Greeted(Ping::contract_digest())),
        // the state on open
        Delivery::Answer(Answer {
            exchange: subscription,
            response: response(),
        }),
        // a change
        Delivery::Answer(Answer {
            exchange: subscription,
            response: response(),
        }),
        // a one-answer exchange opens, answers and ends between changes
        Delivery::Answer(Answer {
            exchange: one_answer,
            response: response(),
        }),
        Delivery::End(Ending::completed(one_answer)),
        // and the subscription goes on answering
        Delivery::Answer(Answer {
            exchange: subscription,
            response: response(),
        }),
    ];

    let mut wire = Vec::new();
    for delivery in &sent {
        wire.write_frame(&delivery.signalize().expect("signalize"), capacity)
            .expect("write");
    }

    let mut stream = Cursor::new(wire);
    let mut subscription_answers = 0_usize;
    let mut one_answer_answers = 0_usize;
    let mut endings = Vec::new();
    let mut greetings = 0_usize;
    for _ in 0..sent.len() {
        let body = stream.read_frame(capacity).expect("read");
        let delivery: Delivery<Response> = Signal::from(body.bytes().to_vec())
            .restore()
            .expect("restore");
        match delivery {
            Delivery::Greeted(_) => greetings += 1,
            Delivery::Answer(answer) if answer.exchange() == subscription => {
                subscription_answers += 1
            }
            Delivery::Answer(answer) if answer.exchange() == one_answer => one_answer_answers += 1,
            Delivery::Answer(answer) => panic!("an answer on exchange {}", answer.exchange()),
            Delivery::End(ending) => endings.push(ending),
        }
    }

    assert_eq!(greetings, 1);
    assert_eq!(subscription_answers, 3, "the state on open and two changes");
    assert_eq!(one_answer_answers, 1);
    assert_eq!(endings, vec![Ending::completed(one_answer)]);
    assert!(
        stream.read_frame(capacity).is_err(),
        "the stream held exactly the frames written"
    );
}

#[test]
fn a_subscriber_the_answering_side_could_not_keep_current_is_told_so() {
    let sent: Delivery<Response> = Delivery::End(Ending::faulted(3, ExchangeFault::Lagged));
    let received = across_the_wire(&sent);
    match received {
        Delivery::End(ending) => {
            assert_eq!(ending.exchange(), 3);
            assert!(!ending.is_connection_wide());
            assert_eq!(
                ending.conclusion,
                Conclusion::Faulted(ExchangeFault::Lagged)
            );
        }
        other => panic!("a lag arrived as {other:?}"),
    }
}

#[test]
fn a_fault_with_no_exchange_to_blame_is_reported_against_the_connection() {
    let ending = Ending::connection_faulted(ExchangeFault::UnreadableQuery);
    assert!(ending.is_connection_wide());
    assert_eq!(ending.exchange(), CONNECTION_EXCHANGE);
    let sent: Delivery<Response> = Delivery::End(ending);
    assert_eq!(across_the_wire(&sent), sent);
}

#[test]
fn a_completed_exchange_and_a_faulted_one_are_distinct_on_the_wire() {
    let completed: Delivery<Response> = Delivery::End(Ending::completed(5));
    let faulted: Delivery<Response> =
        Delivery::End(Ending::faulted(5, ExchangeFault::UnknownExchange));
    assert_ne!(across_the_wire(&completed), across_the_wire(&faulted));
}

/// A contract root shaped like the one the exchange layer must actually
/// carry: `signal-orchestrate`'s `Observed(Locks(Vec<Lock>))`, a variant
/// holding a vector of structs that hold owned strings and a vector of their
/// own. `signal`'s own taxonomy has no vector type, so the shape is declared
/// here rather than left unproven until a consumer port discovers it.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
struct Held {
    identifier: ExchangeId,
    name: String,
    paths: Vec<String>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq)]
enum Observed {
    Holdings(Vec<Held>),
    Nothing,
}

fn holdings() -> Observed {
    Observed::Holdings(vec![
        Held {
            identifier: 1341,
            name: String::from("F6db8dSignalExchangeProtocol"),
            paths: vec![String::from("/git/github.com/LiGoldragon/signal")],
        },
        Held {
            identifier: 1339,
            name: String::from("F6db8dExchangeProtocolDesign"),
            paths: Vec::new(),
        },
    ])
}

#[test]
fn the_envelope_carries_a_root_holding_a_vector_of_string_bearing_structs() {
    let sent: Delivery<Observed> = Delivery::Answer(Answer {
        exchange: 9,
        response: holdings(),
    });
    let received = across_the_wire(&sent);
    match received {
        Delivery::Answer(answer) => {
            assert_eq!(answer.exchange(), 9);
            assert_eq!(answer.response, holdings());
        }
        other => panic!("an observation arrived as {other:?}"),
    }
}

/// The state on open of a subscription whose state is empty must still be
/// answered, or a peer cannot tell an empty state from a state not yet sent.
/// The layer relies on position to mark the state on open, so this is the
/// obligation that reliance places on every streaming contract.
#[test]
fn an_empty_state_on_open_is_still_an_answer_and_not_an_absence() {
    let sent: Delivery<Observed> = Delivery::Answer(Answer {
        exchange: 1,
        response: Observed::Nothing,
    });
    let received = across_the_wire(&sent);
    assert_eq!(received, sent);
    match received {
        Delivery::Answer(answer) => assert_eq!(answer.response, Observed::Nothing),
        other => panic!("an empty state arrived as {other:?}"),
    }
}
