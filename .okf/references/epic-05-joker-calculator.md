---
type: Reference
title: EPIC-05 — Joker Calculator integration
description: Make the Balatro Joker Calculator a thin display over the funky engine — a ScoreTrace emitted by the scoring folds (one step per ScoreOp apply), a BoardSpec written in Rust or YAML, a joker catalog generated from Joker::ALL_JOKERS, and a WIT component — built in the engine's new repo after EPIC-06 moves it out of cardpack.
tags: [epic, funky, balatro, scoring, trace, wit, yaml, active]
resource: https://github.com/ImperialBower/buffoonery/blob/main/docs/EPIC-05_Joker_Calculator.md
timestamp: 2026-10-19T00:00:00Z
---

# What it covers

The first consumer of the [EPIC-01](epic-01-funky.md) solver seam. The engine
gains a `ScoreTrace`: the scoring folds push one `ScoreStep` at every
`ScoreOp::apply`, so `score()` becomes `score_traced().total` and no scoring
logic moves. A `BoardSpec` builds a `BuffoonBoard` from Rust or YAML; a joker
catalog (names, rarity, cost, `MPip`, `Blank` reason) is generated from
`Joker::ALL_JOKERS`; a WIT component (`wasm32-wasip2`, run in the browser via
`jco transpile`) is the boundary. Milestone 0 ran in cardpack; everything after
it runs in the engine's new repo
([funky-moves-to-own-repo](/decisions/funky-moves-to-own-repo.md),
[EPIC-06](epic-06-funky-extraction.md)). The calculator front end lives
outside this repo.

**Moved 2026-10-19.** The EPIC and its design bundle now live in the
`buffoonery` repo: [`docs/EPIC-05_Joker_Calculator.md`](https://github.com/ImperialBower/buffoonery/blob/main/docs/EPIC-05_Joker_Calculator.md) and
`docs/design/balatro-joker-calculator/` there. Cardpack's
`docs/EPIC-05_Joker_Calculator.md` is a pointer; the full text up to the move
is in git at `d41d224`.

# Key rules (see the EPIC)

* **One step per op, not per item.** A played card applies up to 3 ops per
  pass (built-in, special, edition) and a joker 2 (effect, edition); the trace
  records each.
* **The catalog comes from `Joker::ALL_JOKERS`**, never a hand-kept list.
* **Decided 2026-10-18:** fractional `Score` mult (Story S1), the scoring
  folds move out of `board.rs` (Story S2), and the engine moves to its own
  repo (EPIC-06).
* **YAML sets data, Rust adds behavior.** A board and any `MPip` with its
  numbers can be YAML; a new kind of effect needs an `EffectRegistry` in Rust.

# Status

Active since 2026-10-18. The live status is the `## Status` table in the EPIC,
now in `buffoonery`; this page does not track it.

# Paths

* Cardpack: `docs/EPIC-05_Joker_Calculator.md` (pointer only).
* `buffoonery`: `docs/EPIC-05_Joker_Calculator.md`, `docs/design/balatro-joker-calculator/`.

This concept is a pointer, not a copy — the linked documents are authoritative.
