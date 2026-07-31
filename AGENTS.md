# Agent Instructions

Read this repo's `ARCHITECTURE.md` before editing.
Read `NON_IDEAL_AGENTS.md` for temporary debt and workaround guidance before editing.

This repository is a Signal standards library — the second shared `signal-`
crate alongside `signal-frame`.

This crate owns only **genuine cross-component standards**: the reconciled
`ComponentKind` roster, the `Differentiator`, the `AuthorizedObjectInterest`
lattice, and the `ComponentClassification` nameplate. Per Spirit `eeeo`, keep
the scope narrow — it is not a grab-bag of conveniences, not a component's own
contract, and not frame mechanics.

It is a pure vocabulary library. Do not add operation roots, daemon actors,
sockets, redb tables, daemon clients, runtime policy, or a wire codec. The
TrueSchema source lowers through the **declaration-module** emission target
(`build.rs`), not the wire-contract target. Regenerate the checked-in
`src/schema/lib.rs` with `SIGNAL_STANDARD_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`.

`ComponentKind` is closed-but-partitioned: when a real new component appears,
insert it into the right zone's reserved room — do not append blindly or
repartition the zones (a repartition is a major-version event, per `t312`).

This repository is under fast development and constantly breaking.

## Protos estate status

Stack: correct-new destination
Status: active component contract, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
