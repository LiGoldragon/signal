//! Contract identity, and the connection state that holds exchanges open.
//!
//! A contract identifies itself by the digest of the Ethos source it was
//! generated from. `signal-frame` identified a contract by a hand-allocated
//! `ContractId` paired with a `WireRevision` — two registry numbers no
//! registry maintained, which router then never read. A digest needs no
//! registry: a contract computes its own identity from the one artefact that
//! defines it, and two peers agree exactly when their sources agree. This
//! follows the estate's standing rule that identity is trait-borne and an
//! encoded form fingerprints itself.
//!
//! The digest is FNV-1a over the source bytes, computed in a `const fn`, so
//! a contract's identity is fixed when it is compiled rather than hashed at
//! every connection — and the crate every component depends on gains no
//! dependency to carry it. A cryptographic digest was considered and is not
//! warranted: this detects two peers built from different sources, and
//! nothing here defends against a peer choosing its own bytes.
//!
//! Because the digest is exact, a mismatch is a refusal and never a
//! negotiation. There is no accepted-range, no minor-version tolerance and no
//! compatibility path: two peers built from different sources do not talk.

use crate::generated::{
    ContractDigest, ExchangeFault, ExchangeId, Handshake, HandshakeReceipt, HandshakeRejection,
};

use crate::exchange::{FIRST_EXCHANGE, MAXIMUM_OPEN_EXCHANGES};

/// A contract root names the authored Ethos source it was generated from.
///
/// The exception noted here, per the no-namespace rule: the capabilities take
/// no receiver, because a contract's identity belongs to the type and not to
/// any one value of it. The implementor is nonetheless data-bearing — it is
/// the contract's `Query` or `Response` enum — so this is a trait on a real
/// noun and not a namespace.
pub trait Contracted {
    /// The contract's authored Ethos source. Every generated contract crate
    /// already exposes this as a constant beside its projection.
    const CONTRACT_SOURCE: &'static str;

    /// The greeting this side opens a connection with.
    fn greeting() -> Handshake {
        Handshake::of_source(Self::CONTRACT_SOURCE)
    }

    /// The contract's identity: the digest of its authored source.
    fn contract_digest() -> ContractDigest {
        Self::greeting().contract_digest
    }

    /// The receipt this side answers a peer's greeting with. A digest that
    /// does not match is refused, and the refusal carries this side's own
    /// digest so the mismatch is legible from either end.
    fn receipt(greeting: &Handshake) -> HandshakeReceipt {
        let own = Self::contract_digest();
        if greeting.contract_digest == own {
            HandshakeReceipt::Greeted(own)
        } else {
            HandshakeReceipt::GreetingRefused(HandshakeRejection::ContractMismatch(own))
        }
    }
}

/// The FNV-1a offset basis, and the prime it multiplies by. Named here so
/// the digest is legible as the published algorithm rather than as constants.
const DIGEST_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const DIGEST_PRIME: u64 = 0x0000_0100_0000_01b3;

// The exception taken here: a constructor, and one that must be `const` so a
// contract's identity is settled at compile time. The loop is inlined rather
// than calling a hashing helper, because a free function is not available to
// us and a zero-sized hasher type would be a namespace.
impl Handshake {
    /// The greeting a contract generated from `source` opens with.
    pub const fn of_source(source: &str) -> Self {
        let bytes = source.as_bytes();
        let mut digest = DIGEST_BASIS;
        let mut at = 0;
        while at < bytes.len() {
            digest ^= bytes[at] as u64;
            digest = digest.wrapping_mul(DIGEST_PRIME);
            at += 1;
        }
        Self {
            contract_digest: digest as ContractDigest,
        }
    }
}

/// One connection's exchange state: whether it has been greeted, and which
/// exchanges are open on it.
///
/// Both ends keep one. The querying end also mints identifiers from it; the
/// answering end only admits and releases what the querying end names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExchangeLedger {
    next: ExchangeId,
    open: Vec<ExchangeId>,
    capacity: usize,
    greeted: bool,
}

impl Default for ExchangeLedger {
    fn default() -> Self {
        Self {
            next: FIRST_EXCHANGE,
            open: Vec::new(),
            capacity: MAXIMUM_OPEN_EXCHANGES,
            greeted: false,
        }
    }
}

impl From<usize> for ExchangeLedger {
    fn from(capacity: usize) -> Self {
        Self {
            capacity,
            ..Self::default()
        }
    }
}

/// A connection settles its contract once, before any exchange opens.
pub trait Greeted {
    fn is_greeted(&self) -> bool;

    /// Record the greeting. A second greeting on one connection is a fault:
    /// the contract is settled once or not at all.
    fn greet(&mut self) -> Result<(), ExchangeFault>;

    /// Refuse anything that arrives before the greeting.
    fn require_greeting(&self) -> Result<(), ExchangeFault> {
        if self.is_greeted() {
            Ok(())
        } else {
            Err(ExchangeFault::GreetingExpected)
        }
    }
}

/// A connection's exchange state admits, bears and releases exchanges.
pub trait ExchangeTracking {
    fn capacity(&self) -> usize;

    fn open_exchanges(&self) -> &[ExchangeId];

    /// Take an exchange into the open set.
    fn admit(&mut self, exchange: ExchangeId) -> Result<ExchangeId, ExchangeFault>;

    /// Take an exchange out of the open set.
    fn release(&mut self, exchange: ExchangeId) -> Result<ExchangeId, ExchangeFault>;

    fn bears(&self, exchange: ExchangeId) -> bool {
        self.open_exchanges().contains(&exchange)
    }

    /// Refuse a frame naming an exchange this connection does not hold open.
    fn require_open(&self, exchange: ExchangeId) -> Result<ExchangeId, ExchangeFault> {
        if self.bears(exchange) {
            Ok(exchange)
        } else {
            Err(ExchangeFault::UnknownExchange)
        }
    }
}

/// The querying end of a connection mints the identifiers it opens under.
pub trait ExchangeMinting: ExchangeTracking {
    /// The next unused identifier, without opening it.
    fn mint(&mut self) -> Result<ExchangeId, ExchangeFault>;

    /// Mint and admit in one act: how the querying end starts an exchange.
    fn open(&mut self) -> Result<ExchangeId, ExchangeFault> {
        let exchange = self.mint()?;
        self.admit(exchange)
    }
}

impl Greeted for ExchangeLedger {
    fn is_greeted(&self) -> bool {
        self.greeted
    }

    fn greet(&mut self) -> Result<(), ExchangeFault> {
        if self.greeted {
            return Err(ExchangeFault::GreetingRepeated);
        }
        self.greeted = true;
        Ok(())
    }
}

impl ExchangeTracking for ExchangeLedger {
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn open_exchanges(&self) -> &[ExchangeId] {
        &self.open
    }

    fn admit(&mut self, exchange: ExchangeId) -> Result<ExchangeId, ExchangeFault> {
        self.require_greeting()?;
        if self.bears(exchange) {
            return Err(ExchangeFault::ExchangeInUse);
        }
        if self.open.len() >= self.capacity {
            return Err(ExchangeFault::ExchangeLimit);
        }
        self.open.push(exchange);
        Ok(exchange)
    }

    fn release(&mut self, exchange: ExchangeId) -> Result<ExchangeId, ExchangeFault> {
        let at = self
            .open
            .iter()
            .position(|held| *held == exchange)
            .ok_or(ExchangeFault::UnknownExchange)?;
        self.open.remove(at);
        Ok(exchange)
    }
}

impl ExchangeMinting for ExchangeLedger {
    fn mint(&mut self) -> Result<ExchangeId, ExchangeFault> {
        let exchange = self.next;
        self.next = self
            .next
            .checked_add(1)
            .ok_or(ExchangeFault::ExchangeLimit)?;
        Ok(exchange)
    }
}
