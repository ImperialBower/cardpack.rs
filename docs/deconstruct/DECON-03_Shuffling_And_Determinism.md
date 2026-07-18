# DECON-03: Shuffling And Determinism

> **Regeneration spec.** Describes functionality to rebuild, not work landed
> in this repo. Nothing here mandates the original's implementation; source
> citations appear only under Provenance and are non-normative.

## Context

This slice defines **shuffling** a pile: permuting its cards while leaving
the multiset of cards itself unchanged, plus the one determinism property
that makes shuffling testable and reproducible — seeding. It builds on the
pile defined in DECON-02.

Out of scope for this slice: everything about pile construction, drawing,
sorting, and extraction (DECON-02); how a shuffled pile is rendered to or
parsed from a string (DECON-04). This epic is narrowly about permutation
and determinism.

## Status
| Component | Status |
|---|---|
| Multiset-preserving shuffle | Planned |
| Seeded deterministic shuffle | Planned |
| Caller-supplied randomness source | Planned |

## Goals

- Define **shuffle** as an operation that permutes a pile's cards while
  preserving the exact multiset (no card added, dropped, duplicated, or
  mutated).
- Define **seeded shuffle determinism** as a normative property: the same
  seed, applied to the same pile, always yields the same permutation.
- Name the **caller-supplied randomness** escape hatch that lets a
  consumer opt into permutation stability that spans implementations and
  versions, since the seed-to-permutation mapping itself is not portable.

## Scope

- Shuffling a pile never changes which cards it holds or how many of each
  — only their order.
- **Determinism property (normative):** for a fixed seed value and a fixed
  starting pile, shuffling with that seed twice yields byte-identical
  permutations, every time, within one build of one implementation. This
  property — not any particular permutation — is what a rebuild must
  guarantee.
- **Spec decision SD-01:** Should the exact permutation a seed produces be
  a normative, cross-implementation contract? **Options:** pin the exact
  permutation vectors as normative (rebuilds must be bit-compatible with
  the original's specific pseudo-random algorithm) / relax to the
  determinism *property* only (same seed ⇒ same permutation within one
  implementation; the specific permutation is informative evidence, not a
  contract). **Chosen:** relax — the original itself explicitly disclaims
  cross-version stability of its seeded output (a dependency-version bump
  can change every seed's result), so treating its exact permutations as a
  portable, language-agnostic contract would bind rebuilds to an accident
  of one library's algorithm choice that the original's own authors do not
  guarantee even to themselves. A rebuild that *does* need
  cross-implementation-identical shuffles achieves it by accepting a
  **caller-supplied randomness source** — the consumer picks and fixes a
  named, portable algorithm themselves, outside this pack's scope, and the
  pile shuffle operation simply consumes whatever sequence that source
  produces.
- `vectors/shuffling/seeded-shuffle.json` is **informative** evidence of
  the above: it records the same-seed-twice repeatability check plus the
  literal permutations produced by three example seeds (42, 1337, 2026)
  against one deck vocabulary, at the pinned source commit. A rebuild is
  not required to reproduce these exact permutations; it is required to
  reproduce the *repeatability* they demonstrate and the *multiset
  preservation* every one of them exhibits.

## Domain map
| Concept | Required behavior | Vectors |
|---|---|---|
| Shuffle | Permutes a pile, preserves the multiset of cards | `vectors/shuffling/seeded-shuffle.json` (informative, SD-01) |
| Seeded shuffle determinism | Same seed ⇒ identical permutation, repeatably | `vectors/shuffling/seeded-shuffle.json` (`seed_42_repeatable`) |
| Caller-supplied randomness | Consumer-fixed algorithm gives cross-implementation reproducibility | — (no vector; the mechanism itself is the consumer's choice, see Not specified) |

## Design

Shuffle is a permutation operation on a pile's existing cards — it neither
consults nor can alter the deck vocabulary those cards came from (the same
value-semantics guarantee as DECON-01 and DECON-02).

The determinism property is the load-bearing contract: it is what lets a
rebuild's own tests, and any downstream consumer's tests, treat "shuffle
with seed N" as a repeatable fixture rather than a source of test flake.
It does **not** require that two different implementations (or two
different versions of the same implementation) agree on *which*
permutation a given seed produces — only that one implementation, at one
point in time, is self-consistent.

## Perspectives
| Perspective | May | Must not | Boundary invariant |
|---|---|---|---|
| God-mode | — | — | N/A for this slice. |
| Administrative | — | — | N/A for this slice. |
| User/client | Shuffle their own pile, with or without a seed, with or without a caller-supplied randomness source | Alter the deck vocabulary via a shuffle | Shuffling mutates only the calling pile instance's card order; the multiset and the vocabulary are untouched. |
| Observer/operator *(Partial)* | Observe shuffle failures, if any are defined | — | Routine shuffles are silent in the original — no trace, log, or event marks a shuffle happening on the happy path. This is recorded as a gap, not a requirement: a rebuild may improve on it (emit a shuffle event) without breaking this spec; doing so is purely additive and informative, not binding. |

## Work Items
### Phase 0 — Multiset-preserving shuffle
- [ ] **0a.** Permute a pile's cards with no fixed seed; confirm the
  resulting pile holds the exact same multiset of cards as before.

### Phase 1 — Seeded deterministic shuffle
- [ ] **1a.** Shuffle a pile with a given seed twice from the same
  starting pile; confirm the two permutations are identical. Proven by
  `vectors/shuffling/seeded-shuffle.json` (`seed_42_repeatable`).
- [ ] **1b.** Confirm a seeded shuffle also preserves the multiset (same
  guarantee as Phase 0, seeded case).

### Phase 2 — Caller-supplied randomness source
- [ ] **2a.** Accept a randomness source supplied by the caller (rather
  than sourced internally) to drive a shuffle, so a consumer needing
  cross-implementation-identical results can fix their own portable
  algorithm.

## Test Plan

- **Given** a 52-card pile and seed 42, **when** shuffled twice
  independently from the same starting pile, **then** both permutations
  are byte-identical index strings, matching `seed_42_repeatable.first`
  and `.second` in `vectors/shuffling/seeded-shuffle.json`.
- **Given** a 52-card pile shuffled with any seed, **when** the resulting
  pile's cards are compared as an unordered set against the starting
  pile's cards, **then** they are identical (no card added, dropped, or
  duplicated). Evidenced informatively by every permutation in
  `vectors/shuffling/seeded-shuffle.json`.
- **Given** a caller-supplied randomness source, **when** used to drive a
  shuffle, **then** the resulting pile's cards remain the same multiset as
  before (same guarantee, different randomness origin).

## Not specified (implementer's choice)

- The pseudo-random algorithm used internally for shuffling — pinned or
  not, it is not part of this contract (SD-01: relaxed).
- The exact mapping from a seed value to a permutation — informative only;
  `vectors/shuffling/seeded-shuffle.json`'s literal permutations are
  evidence, not a target to reproduce byte-for-byte.
- The type/width of a seed value (the original uses a 64-bit unsigned
  integer; a rebuild may use any type capable of selecting a deterministic
  starting state).
- The source of randomness for an unseeded shuffle (process entropy, OS
  randomness, a runtime-specific source, …).
- The exact interface shape for a caller-supplied randomness source
  (an injected object, a function pointer, a stream/iterator, …) — only
  that one exists and that it is consulted instead of any internal source
  when provided.
- Whether shuffle events are observable (logged, traced, emitted) — the
  original emits nothing on the happy path; a rebuild may add this without
  violating the spec.

## Spec decisions

- **SD-01** (this epic): Seeded-shuffle permutations — relaxed. The
  determinism *property* (same seed ⇒ same permutation, within one
  implementation) is normative; the exact permutation vectors in
  `vectors/shuffling/seeded-shuffle.json` are informative only. See Scope,
  above, for the full decision text.

## Verification

Any implementation must reproduce the repeatability and multiset-
preservation demonstrated by `vectors/shuffling/seeded-shuffle.json`:
1. Shuffling the same starting pile twice with the same seed yields two
   identical permutations (`seed_42_repeatable.equal == true`).
2. Every seeded permutation in the vector's `shuffles` list, when compared
   as an unordered set against the starting pile, contains the exact same
   cards.
3. The literal permutation strings in the vector are **not** required to
   match byte-for-byte (SD-01); reproducing them exactly is allowed but
   not a pass/fail criterion.

## Dependencies

**Builds on:** DECON-02. **Blocks:** —

## Provenance (non-normative)

- Unseeded shuffle: `src/basic/types/pile.rs:734-748` (process-default
  randomness source).
- Seeded shuffle and its documented cross-version-stability disclaimer:
  `src/basic/types/pile.rs:750-779`; repeatability doctest `:761-764`.
- Caller-supplied randomness source: `src/basic/types/pile.rs:781-796`.
- Multiset preservation: `tests/properties.rs:30` (property test, any
  input pile); named unit test
  `src/basic/types/pile.rs:1632-1640` (`shuffled_with_seed__same_cards`).
- Determinism property test (any seed, not just fixed values):
  `tests/properties.rs:37`; fixed-seed-42 case
  `src/basic/types/pile.rs:1614`; wasm-target equivalent
  `tests/wasm.rs:34`.
- Observer gap (routine shuffles produce no log/trace output): confirmed
  by absence — the only `log::` call sites in the crate are on
  malformed-input failure paths (`src/basic/types/pile.rs:333`,
  `src/localization.rs:238`, `src/basic/decks/razz.rs:9,31`), none on a
  shuffle's happy path.
