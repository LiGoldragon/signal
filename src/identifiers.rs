//! Readable, nominal identifiers for local, cluster, and public references.
//!
//! `ethos/identifiers.ethos` is the schema for the three fixed-width forms.
//! Datom's available intrinsics at this revision are `String` and `Integer`,
//! with no standard hash primitive. `NameDigest` is therefore a Signal-owned
//! BLAKE3 adapter, not a claim about a Datom hash type or authentication.

use bip39::Language;
use std::fmt;

pub const BIP39_WORDS: usize = 2048;
pub const BITS_PER_WORD: usize = 11;
pub const LOCAL_WORDS: usize = 3;
pub const CLUSTER_WORDS: usize = 6;
pub const PUBLIC_WORDS: usize = 12;
pub const LOCAL_BITS: usize = LOCAL_WORDS * BITS_PER_WORD;
pub const CLUSTER_BITS: usize = CLUSTER_WORDS * BITS_PER_WORD;
pub const PUBLIC_BITS: usize = PUBLIC_WORDS * BITS_PER_WORD;

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct NameDigest(pub [u8; 32]);

#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct LocalNameReference([u16; LOCAL_WORDS]);
#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct ClusterNameReference([u16; CLUSTER_WORDS]);
#[derive(
    rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash,
)]
pub struct PublicNameReference([u16; PUBLIC_WORDS]);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NameParseError {
    Empty,
    NonCanonical,
    UnknownWord(String),
    WrongLength { expected: usize, actual: usize },
    InvalidIndex(u16),
}

impl NameDigest {
    pub fn of_bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self(*blake3::hash(bytes.as_ref()).as_bytes())
    }
    pub fn local(self) -> LocalNameReference {
        LocalNameReference(words(&self.0, LOCAL_WORDS).try_into().expect("fixed width"))
    }
    pub fn cluster(self) -> ClusterNameReference {
        ClusterNameReference(
            words(&self.0, CLUSTER_WORDS)
                .try_into()
                .expect("fixed width"),
        )
    }
    pub fn public(self) -> PublicNameReference {
        PublicNameReference(
            words(&self.0, PUBLIC_WORDS)
                .try_into()
                .expect("fixed width"),
        )
    }
}

impl LocalNameReference {
    pub fn try_from_indices(indices: [u16; LOCAL_WORDS]) -> Result<Self, NameParseError> {
        validate(&indices).map(|_| Self(indices))
    }
    pub fn parse(text: &str) -> Result<Self, NameParseError> {
        parse_words(text, LOCAL_WORDS).map(|words| Self(words.try_into().expect("fixed width")))
    }
}
impl ClusterNameReference {
    pub fn try_from_indices(indices: [u16; CLUSTER_WORDS]) -> Result<Self, NameParseError> {
        validate(&indices).map(|_| Self(indices))
    }
    pub fn parse(text: &str) -> Result<Self, NameParseError> {
        parse_words(text, CLUSTER_WORDS).map(|words| Self(words.try_into().expect("fixed width")))
    }
}
impl PublicNameReference {
    pub fn try_from_indices(indices: [u16; PUBLIC_WORDS]) -> Result<Self, NameParseError> {
        validate(&indices).map(|_| Self(indices))
    }
    pub fn parse(text: &str) -> Result<Self, NameParseError> {
        parse_words(text, PUBLIC_WORDS).map(|words| Self(words.try_into().expect("fixed width")))
    }
}

impl fmt::Display for LocalNameReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display(&self.0, f)
    }
}
impl fmt::Display for ClusterNameReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display(&self.0, f)
    }
}
impl fmt::Display for PublicNameReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        display(&self.0, f)
    }
}

fn words(bytes: &[u8; 32], count: usize) -> Vec<u16> {
    (0..count)
        .map(|word| {
            let bit = word * BITS_PER_WORD;
            let mut value = 0u16;
            for offset in 0..BITS_PER_WORD {
                value = (value << 1)
                    | u16::from((bytes[(bit + offset) / 8] >> (7 - ((bit + offset) % 8))) & 1);
            }
            value
        })
        .collect()
}

fn display(indices: &[u16], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (position, index) in indices.iter().enumerate() {
        let word = Language::English
            .word_list()
            .get(usize::from(*index))
            .ok_or(fmt::Error)?;
        let mut chars = word.chars();
        let first = chars.next().expect("BIP39 word");
        write!(
            f,
            "{}{}",
            if position == 0 {
                first
            } else {
                first.to_ascii_uppercase()
            },
            chars.as_str()
        )?;
    }
    Ok(())
}

fn parse_words(text: &str, expected: usize) -> Result<Vec<u16>, NameParseError> {
    if text.is_empty() {
        return Err(NameParseError::Empty);
    }
    if !text.is_ascii()
        || text.contains(|c: char| !c.is_ascii_alphabetic())
        || !text.as_bytes()[0].is_ascii_lowercase()
    {
        return Err(NameParseError::NonCanonical);
    }
    let starts = text
        .char_indices()
        .filter_map(|(at, character)| (at == 0 || character.is_ascii_uppercase()).then_some(at))
        .collect::<Vec<_>>();
    let pieces = starts
        .iter()
        .enumerate()
        .map(|(position, start)| {
            let end = starts.get(position + 1).copied().unwrap_or(text.len());
            let word = &text[*start..end];
            format!("{}{}", word[..1].to_ascii_lowercase(), &word[1..])
        })
        .collect::<Vec<_>>();
    if pieces.len() != expected {
        return Err(NameParseError::WrongLength {
            expected,
            actual: pieces.len(),
        });
    }
    pieces
        .into_iter()
        .map(|word| {
            Language::English
                .find_word(&word)
                .map(|index| index as u16)
                .ok_or(NameParseError::UnknownWord(word))
        })
        .collect()
}

fn validate(indices: &[u16]) -> Result<(), NameParseError> {
    indices
        .iter()
        .find(|index| usize::from(**index) >= BIP39_WORDS)
        .copied()
        .map_or(Ok(()), |index| Err(NameParseError::InvalidIndex(index)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn widths_are_explicit() {
        assert_eq!((LOCAL_BITS, CLUSTER_BITS, PUBLIC_BITS), (33, 66, 132));
    }
    #[test]
    fn display_round_trips_each_context() {
        let digest = NameDigest::of_bytes(b"signal identifier proof");
        assert_eq!(
            LocalNameReference::parse(&digest.local().to_string()).unwrap(),
            digest.local()
        );
        assert_eq!(
            ClusterNameReference::parse(&digest.cluster().to_string()).unwrap(),
            digest.cluster()
        );
        assert_eq!(
            PublicNameReference::parse(&digest.public().to_string()).unwrap(),
            digest.public()
        );
    }
    #[test]
    fn rejects_noncanonical_and_unknown_words() {
        assert_eq!(
            LocalNameReference::parse("AbandonAbilityAble"),
            Err(NameParseError::NonCanonical)
        );
        assert_eq!(
            LocalNameReference::parse("wibbleAbilityAble"),
            Err(NameParseError::UnknownWord("wibble".into()))
        );
    }
    #[test]
    fn nominal_reference_archives_and_restores() {
        let value = NameDigest::of_bytes(b"archive").cluster();
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap();
        assert_eq!(
            rkyv::from_bytes::<ClusterNameReference, rkyv::rancor::Error>(&bytes).unwrap(),
            value
        );
    }
    #[test]
    fn rejects_bad_indices_without_display_panic() {
        assert_eq!(
            LocalNameReference::try_from_indices([BIP39_WORDS as u16, 1, 2]),
            Err(NameParseError::InvalidIndex(BIP39_WORDS as u16))
        );
    }
}
