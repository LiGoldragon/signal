# Word identifiers proof

`ethos/identifiers.ethos` is the standalone `Identifier` library: a full digest shape and three fixed-width reference shapes. It stays outside `ethos/signal.ethos`, so this proof does not alter Signal's frame ABI or generated taxonomy.

`src/identifiers.rs` provides rkyv-serializable nominal `NameDigest`, `LocalNameReference`, `ClusterNameReference`, and `PublicNameReference` types. None is a `String` internally. The full digest is BLAKE3 over input bytes; it is a correlation/deduplication digest and makes no authentication claim.

The chosen alphabet is BIP-39 English: its pinned crate supplies 2,048 CC0 English words, or 11 bits per word. BIP-39 mnemonic checksums are not used.

| Context | Words | Bits | Capacity |
|---|---:|---:|---:|
| Local/private | 3 | 33 | 2^33 |
| Cluster | 6 | 66 | 2^66 |
| Public | 12 | 132 | 2^132 |

The display form is CamelCase BIP-39 words, such as `AbandonAbilityAble`. Parsing accepts only ASCII alphabetic CamelCase with BIP-39 English words and the exact context width. It rejects lowercase, colon, dots, other punctuation, unknown words, and noncanonical counts. The display is a short reference; the full 256-bit `NameDigest` is the explicit long form.

Pinned Datom and Protos provide text and integer intrinsics but no standard hash primitive. The Ethos library uses `Integer` limbs and the Signal-owned BLAKE3 adapter fills that gap without claiming a Datom standard or editing Datom.

`examples/name_identifier.rs` prints each form and full digest. Tests cover widths, display/parser round trips, invalid forms, and rkyv archive restoration.
