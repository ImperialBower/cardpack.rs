---
type: Decision
title: The funky engine moves to its own repo (buffoonery)
description: The Balatro-style engine leaves cardpack and becomes the foundation of a solver repo (WIT boundary, YAML or Rust configuration) that depends on cardpack; cardpack deprecates the funky feature in 0.11.2 and removes it in 0.12.0. Do not add engine features to cardpack.
tags: [decision, funky, purity, repo-boundary, wit]
timestamp: 2026-10-18T00:00:00Z
---

# Decision

The [funky engine](/architecture/funky-engine.md) (`src/funky/`) **moves out
of cardpack** into its own repo, `buffoonery`. That repo becomes the foundation of a Balatro
solver: it has a WIT (WebAssembly Component Model) boundary, it can be
configured from YAML or from Rust, and it depends on a published cardpack.
Cardpack **deprecates** its `funky` feature in 0.11.2 and **removes** it in
0.12.0. Cardpack cannot re-export the engine afterwards: the dependency would
be a cycle. Decided 2026-10-18 while scoping
[EPIC-05](/references/epic-05-joker-calculator.md) (its item 0h); the move is
planned in [EPIC-06](/references/epic-06-funky-extraction.md).

# Why

* **Cardpack has no game state; the engine is a game.** Cardpack is a card
  value kernel ([domain kernel](/architecture/domain-kernel.md)). The engine
  models boards, money, rounds, a shop and scoring. Without it, the
  domain-kernel description is simply true.
* **The solver's changes are engine changes.** EPIC-05's keystone work
  (`ScoreTrace`, a fractional `Score`, splitting the scoring folds) all edits
  the engine. With the engine in cardpack and the solver elsewhere, every such
  change would need a cardpack release first.
* **WIT fits a solver, not cardpack.** Cardpack's verdict against a WIT
  boundary rests on it having no state machine. That reason does not apply to
  a solver, so the verdict stands for cardpack and does not bind the new repo.
* **The cut is clean.** The engine uses only public cardpack items, and
  `Pip`'s fields are public, so the joker `const`s compile outside the crate.
* **It avoids a known trap.** A `funky-wasm` workspace member inside cardpack
  could have poisoned cargo-deny's feature resolve, the way a self
  dev-dependency once did ([examples-flag-free-alias](/decisions/examples-flag-free-alias.md)).

# How to apply

* **Do not add engine features to cardpack.** New Balatro work goes to the new
  repo. Fixes to cardpack's public card types that the engine needs are fine.
* Until 0.12.0, `src/funky/` stays buildable and tested in cardpack, but only
  for the deprecation window.
* The repo and crate are named **`buffoonery`** (`funky` and `buffoon` are taken on crates.io).

# Citations

* `docs/EPIC-06_Funky_Extraction.md` — the move, phase by phase.
* `docs/EPIC-05_Joker_Calculator.md` — decisions 0f–0h and open question 2.
* `src/lib.rs:360-366` — the two feature-gated modules that go away.
* `src/basic/types/pips.rs:79-85` — `Pip`'s public fields.
