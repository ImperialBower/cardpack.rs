# EPIC-01 Funky — Progress Report & Quality Evaluation

**Date:** 2026-10-18
**Evaluated at:** `origin/main` tip `a2bdc82` (merge of PR #108, the `thiserror`
2.0.21 bump). Funky reached `main` through PR #74 (`a455b7b`, 2026-07-18), so
`main` is now the only home of this code; `origin/funky` (`1db3bea`) has
nothing `main` lacks.
**Note:** measured on a clean `git archive a2bdc82` export, not the working
tree. The local `main` (`63b7f64`) is one merge behind `origin/main`.

> This replaces the first evaluation (2026-07-05, at `cc1595d`, grade C+). That
> snapshot is kept in git at `84a20e2` — read it there for the pre-close-out
> picture. The [Since the last evaluation](#since-the-last-evaluation) table
> below lists what changed.

---

## Executive summary

**Status: shipped, then dormant in maintenance mode.** EPIC-01 and all five
children (01a–01e) closed between 2026-07-16 and 2026-07-18
([EPIC-01_Funky.md](EPIC-01_Funky.md) corrigendum), and funky merged to `main`
the same day. Both stated goals are met in structure: the solver path
(`score()` / `score_with_seed()`, four phases, never panics) and the modding
path (`Effect` / `EffectRegistry`, `src/funky/types/effect.rs`). Every check
below is green at `a2bdc82`. Since close-out there has been **no feature work**
on funky — only fixes forced by dependency and lint changes.

The open work is the deferral register in the parent EPIC (Seals, Antes &
bosses, Decks, Tags, draw-step hooks, edition sourcing, Spectral pack,
Supernova / Loyalty Card). Nothing is half-built. What remains is hygiene: a
journal voice in rustdoc, one 8,342-line file, and a few stale doc comments.

**Overall grade: A− / "finished muscle, some journal ink on the skin."** It was
C+ on 2026-07-05.

---

## Timeline & activity

| Period | Activity |
|---|---|
| 2025-03 → 2025-04 | Main build-out (BuffoonCard, decks, jokers, planets, hand detection, scoring phases) |
| 2025-09 | Short burst (capacity, Half Joker, phase-4 joker scoring) |
| 2026-07-05 | First evaluation (C+) at `cc1595d` |
| 2026-07 | **65 commits on `src/funky`**: CI hardening, all four phases, 01a–01e, close-out at `e50fdd0` (07-18), merge to `main` via PR #74 |
| 2026-07-23 | `funky` now implies `serde` (`Cargo.toml:36`), so the funky examples build with the feature alone |
| 2026-08 | 1 funky commit: `883599f` — `rand` 0.10 moved `random_range` to `RngExt`; 32 call sites in `board.rs` and `basic/types/pile.rs` broke and were fixed |
| 2026-10 | 1 funky commit: `fdd5246` — assert messages for the new `clippy::assert_is_empty` lint |
| 2026-07-18 → now | No feature work (~3 months) |

Since 2026-07-05, two author identities touched `src/funky`: folkengine (35
commits) and Christoph Baker (32).

## Size & shape

- **17,950 lines across 22 files** in `src/funky` (was 5,778 across 17).
- The weight sits in one file: `src/funky/types/board.rs` is **8,342 lines**.
  About 3,180 are production code; the `#[cfg(test)]` module starts at
  `board.rs:3180`, so the other ~5,160 are its 311 tests.
- Next largest: `decks/joker.rs` (3,185), `types/buffoon_pile.rs` (1,236),
  `types/mpip.rs` (779), `decks/basic.rs` (779), `types/buffoon_card.rs` (775).
- Still layered on the core: `BuffoonPile` implements the core `Ranged` trait,
  so hand analysis reuses the `basic` engine. Still feature-gated and
  `std`-only: `funky = ["std", "serde"]` (`Cargo.toml:36`). The no_std kernel
  build stays clean.
- `phf` is gone from `Cargo.toml`. `fpips.rs` is deleted.

## Verification results (this evaluation, at `a2bdc82`)

| Check | Result |
|---|---|
| `cargo test --locked --features full,funky` (the CI line, `CI.yaml:41`) | ✅ **893 lib tests**, 48 integration tests in 5 binaries, **132 doctests** (11 ignored). 0 failed |
| `cargo test --locked --features full,funky --lib funky` | ✅ **465 funky lib tests** (was 383 for the whole crate on 2026-07-05) |
| `cargo clippy --locked --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic` (`CI.yaml:81`) | ✅ **0 warnings, 0 errors** |
| `cargo build --locked --no-default-features` | ✅ clean |
| `cargo fmt --all -- --check` | ✅ clean |
| `cargo run --features funky --example buffoon` | ✅ final score **11254** (same as close-out) |
| `cargo run --features funky --example funky_tour` | ✅ runs |
| CI coverage of funky | ✅ gated: tests at `CI.yaml:41`, pedantic clippy at `CI.yaml:81`, doc and wasm builds with `--all-features` (`CI.yaml:109`, `:121`) |

Not run here: `cargo llvm-cov`, `cargo mutants`, the MSRV 1.85 job. CI runs
them; this evaluation did not.

## What works today

A caller with `--features funky` can:

1. Build Basic 52, Abandoned 40 and Checkered decks (`src/funky/decks/basic.rs`).
2. Detect every Balatro hand type, with the `HandRules` seam for Four Fingers,
   Shortcut and Smeared.
3. Level hands with the 12 planet cards (`src/funky/decks/planet.rs`).
4. Score a full four-phase hand with no panics (`board.rs` holds no `todo!()`,
   `unimplemented!()` or non-test `panic!`). `score()` is deterministic;
   `score_with_seed` rolls Lucky, Misprint, Glass and spectral creation the same
   way every time for one seed.
5. Use **104 of 112 jokers**. The other 8 stay `Blank`, each with a reason a
   test enforces (`BLANK_WITH_REASON`, `src/funky/decks/joker.rs`).
6. Apply card-enhancing tarots, 14 of 18 spectrals, editions (Negative as a
   live slot exemption), 20 vouchers, the shop (stock, rerolls, packs, selling)
   and 3 boss blinds.
7. Add custom effects through `MPip::Custom(u32)` and `score_with_registry`
   (`src/funky/types/effect.rs`).

It **cannot** yet: use seals, run ante progression or most boss blinds, use 13
of the 16 Balatro decks, use tags, or score Supernova and Loyalty Card. These
are all named deferrals in the parent EPIC, not bugs.

## Since the last evaluation

Each "Recommended next step" from 2026-07-05, and what happened to it:

| 2026-07-05 finding | Now |
|---|---|
| CI does not compile funky | ✅ Fixed — `CI.yaml:41` and `:81` |
| Phases 1–3 are `todo!()` and panic | ✅ Fixed — all four phases score |
| `println!` in scoring paths | ✅ Fixed — no `print!`, `println!` or `dbg!` in `src/funky` |
| 6 clippy warnings | ✅ Fixed — 0 at pedantic, all targets |
| `examples/buffoon.rs` is a stub | ✅ Fixed — four-phase demo, 67 lines; plus `funky_tour.rs` (153) |
| `joker.rs` and `decks/basic.rs` have zero tests | ✅ Fixed — 19 and 8 tests, plus crate-wide reachability guards |
| ~54 `MPip` variants silently score 0 | ✅ Mostly fixed — guarded by tests; 2 variants remain (Supernova, Loyalty Card) |
| `phf` unused; mod design undecided | ✅ Fixed — `phf` removed; `EffectRegistry` chosen |
| `Draws` / `ToggleCard` not exported | ✅ Fixed — in the funky prelude |
| Journal-voice rustdoc ("DIARY") | ❌ Still there — see Weaknesses |
| Joker named `HACK` flagged as debris | ✔️ **Retracted** — Hack is a real Balatro joker (retriggers 2–5). `joker.rs:701` is correct |

## Quality evaluation

**Strengths**

- **The silent-zero risk is now closed by tests, not by care.** The `_ => 0`
  match arms still exist (for example `src/funky/types/buffoon_card.rs:155`,
  `:165`), but an exhaustive per-variant match in `joker.rs` and two
  reachability guards fail if a new joker scores 0 without being listed. This
  was the worst failure mode for a solver, and it now fails loudly.
- **"Not done" and "waiting on a subsystem" can be told apart in the source.**
  `BLANK_WITH_REASON` names the missing subsystem for each of the 8 unwired
  jokers.
- **CI works as a license to be fallible.** Two dependency bumps broke funky
  after close-out: `rand` 0.10 (`883599f`, 32 call sites) and the MSRV 1.85
  let-chains (`637aa95`). Both were fixed in follow-up commits. On 2026-07-05,
  with funky outside CI, the `rand` break would not have shown up there.
- **Tests carry exact wiki values.** Joker tests assert Balatro's own numbers,
  so a wrong number fails a test instead of looking plausible.
- **Clean feature isolation stays intact** after the merge into a `main` that
  also holds the no_std kernel and the sealed-deck work.

**Weaknesses**

- **Journal voice in published rustdoc.** Seven `DIARY` notes are still in doc
  comments: `src/funky/types/buffoon_pile.rs:90`, `:100`, `:309`, `:417`,
  `:807`, `:942`, and `src/funky/decks/joker.rs:794`. Some read badly in
  rendered docs ("I really don't want to write any more tests",
  `buffoon_pile.rs:942`). This is the only finding left from 2026-07-05.
- **One file holds the engine.** `board.rs` is 8,342 lines: round loop,
  scoring, shop, vouchers, editions, spectrals and 311 tests. It works, but
  each new subsystem (Seals, Antes) will make it bigger. `src/funky/types/shop.rs`
  (74 lines) holds only data types; all shop behaviour and tests live in
  `board.rs`.
- **Stale rustdoc.** `board.rs:533-536` says `MultTimesOnEmptyJokerSlots`
  (Joker Stencil) "is not applied yet". It is applied at `board.rs:879`.
  The same note about `MultTimesEveryXHands` is still true.
- **Two variants still score 0.** `MultPlusOnHandPlays` (Supernova,
  `joker.rs:829`) and `MultTimesEveryXHands` (Loyalty Card, `joker.rs:540`) sit
  in the guard's exclusion list (`joker.rs:2591`, `:2593`). The parent EPIC
  says the `hands_played` counter they need already exists.
- **The deferral register is partly out of date.** It lists "Serde on funky
  types" as deferred. But 13 files under `src/funky/types/` already use serde's
  `Serialize` (for example the derive at `src/funky/types/score.rs:5-7`), and
  `funky` turns on `serde` since 2026-07-23. What is really missing is
  narrower: no funky test runs a serde round trip, and custom effects have no
  stable string ids.

## Risk register

| Risk | Severity | Note |
|---|---|---|
| `board.rs` keeps growing | Medium | Seals and Antes both touch scoring. Without a split, review and merge get harder |
| Funky serde shape is untested | Medium | Types serialize, but nothing pins the format. A field rename silently breaks saved state |
| Dependency churn breaks funky | Low | Happened twice since close-out. Funky is now in CI, so a break shows up as a red build |
| Journal-voice rustdoc | Low | Looks unprofessional on docs.rs; no runtime effect |
| Stale rustdoc on `board.rs:533` | Low | Misleads a reader into thinking Stencil is unwired |
| Knowledge decay (3 months dormant) | Low | The EPIC chain and corrigenda are thorough, which lowers the cost of a restart |

## Recommended next steps (priority order)

1. **Fix the stale comment at `board.rs:533-536`.** One line. Keep only the
   `MultTimesEveryXHands` part.
2. **Rewrite the 7 `DIARY` doc comments as reference docs.** Move the journal
   text to a devlog if you want to keep it.
3. **Wire Supernova and Loyalty Card.** The parent EPIC names them as the first
   items of any follow-on sweep, and the counter exists. This removes the last
   two entries from the exclusion list.
4. **Reconcile the serde deferral** in the parent EPIC's register. Then add one
   round-trip test per public funky type, so the format is pinned before anyone
   saves state with it.
5. **Split `board.rs` before the next subsystem EPIC.** A natural cut is by
   phase of the round: scoring, shop and economy, consumables. Do it as a
   no-behaviour-change refactor with the 311 tests as the safety net.
6. **Start the next subsystem EPIC.** Seals is the last ❌ in the domain map and
   unblocks 4 spectrals and red-seal retriggers.

## Relationship to EPIC-01 / other work

This document evaluates the code. [EPIC-01_Funky.md](EPIC-01_Funky.md) is the
authoritative status and holds the deferral register. The children are
[01a Joker Wiring](EPIC-01a_Joker_Wiring_Backlog.md),
[01b Shop](EPIC-01b_Shop.md), [01c Vouchers](EPIC-01c_Vouchers.md),
[01d Editions](EPIC-01d_Editions.md) and
[01e Spectral Cards](EPIC-01e_Spectral_Cards.md).

There is still no technical link to [EPIC-02 Ganjifa](EPIC-02_Ganjifa.md) or the
[EPIC-04 sealed-deck family](EPIC-04_Sealed_Decks.md). Those build on the
no_std `basic` core; funky is a std-only layer on top of it.
