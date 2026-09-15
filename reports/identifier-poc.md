# Word identifiers proof

`ethos/identifiers.ethos` is a native sweet-form `Library` with imports, types, kinds, and associations sections. It declares the three fixed-width reference shapes and stays outside `ethos/signal.ethos`, so this proof does not alter Signal's frame ABI or generated taxonomy.

`src/identifiers.rs` provides rkyv-serializable nominal `NameDigest`, `LocalNameReference`, `ClusterNameReference`, and `PublicNameReference` types. None is a `String` internally. The full digest is BLAKE3 over input bytes; it is a correlation/deduplication digest and makes no authentication claim.

The chosen alphabet is BIP-39 English: its pinned crate supplies 2,048 CC0 English words, or 11 bits per word. BIP-39 mnemonic checksums are not used.

| Context | Words | Bits | Capacity |
|---|---:|---:|---:|
| Local/private | 3 | 33 | 2^33 |
| Cluster | 6 | 66 | 2^66 |
| Public | 12 | 132 | 2^132 |

The display form is lowerCamelCase BIP-39 words, such as `abandonAbilityAble`. Parsing accepts only ASCII alphabetic lowerCamelCase with BIP-39 English words and the exact context width. It rejects PascalCase, colon, dots, other punctuation, unknown words, invalid word indices, and noncanonical counts. The display is a short reference; the full 256-bit `NameDigest` is the explicit long form.

Pinned Datom and Protos provide text and integer intrinsics but no standard hash primitive. The Ethos library therefore models only the word-index references; the Signal-owned 32-byte BLAKE3 `NameDigest` is an explicit adapter outside that schema, without claiming a Datom standard or editing Datom.

Real `tiktoken 0.12.0` measurements of the running example's exact lowerCamelCase output: cl100k_base and o200k_base both count local (33 bits) as 3 tokens, cluster (66 bits) as 8 tokens, and public (132 bits) as 17 tokens. These are sample-string measurements, not a claim about the production Codex tokenizer.

`examples/name_identifier.rs` prints each form and full digest. Tests cover widths, display/parser round trips, invalid forms, and rkyv archive restoration.
