---
type: Reference
title: EPIC-05 — Joker Calculator integration
description: Make the Balatro Joker Calculator a thin display over funky — a ScoreTrace emitted by the scoring folds (one step per ScoreOp apply), a serde BoardSpec, a joker catalog generated from Joker::ALL_JOKERS, and a separate funky-wasm adapter crate.
tags: [epic, funky, balatro, scoring, trace, wasm, active]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/EPIC-05_Joker_Calculator.md
timestamp: 2026-10-18T00:00:00Z
---

# What it covers

The first consumer of the [EPIC-01](epic-01-funky.md) solver seam. The kernel
gains a `ScoreTrace` (in `src/funky/types/trace.rs`): the scoring folds push
one `ScoreStep` at every `ScoreOp::apply`, so `score()` becomes
`score_traced().total` and no scoring logic moves. A `BoardSpec` builds a
`BuffoonBoard` from JSON-friendly data; a joker catalog (names, rarity, cost,
`MPip`, `Blank` reason) is generated from `Joker::ALL_JOKERS`. All JSON and
`wasm-bindgen` code lives in a separate `funky-wasm` adapter, never in
`src/funky/` ([domain kernel](/architecture/domain-kernel.md)). The calculator
front end lives outside this repo; its design bundle is in
`docs/design/balatro-joker-calculator/`.

# Key rules (see the EPIC)

* **One step per op, not per item.** A played card applies up to 3 ops per
  pass (built-in, special, edition) and a joker 2 (effect, edition); the trace
  records each.
* **The catalog comes from `Joker::ALL_JOKERS`**, never a hand-kept list.
* **Three decisions gate Story 1**: the `Score` type, where the scoring folds
  live, and where `funky-wasm` lives (EPIC Milestone 0f–0h, which records the
  outcome). For `funky-wasm`, mind the
  [examples-flag-free-alias](/decisions/examples-flag-free-alias.md)
  cargo-deny trap.

# Status

Active since 2026-10-18. The live status is the `## Status` table in
`docs/EPIC-05_Joker_Calculator.md`; this page does not track it.

# In-repo paths

`docs/EPIC-05_Joker_Calculator.md`, `docs/design/balatro-joker-calculator/`

This concept is a pointer, not a copy — the linked documents are authoritative.
