# EPIC-01 Funky — Progress Report & Quality Evaluation

**Date:** 2026-07-05
**Evaluated at:** `origin/funky` tip `cc1595d` (merge of origin/main 0.7.0 no_std into funky), via detached worktree.
**Note:** the local `funky` branch is **20 commits behind** `origin/funky` — the missing commits are the main-merge (`cc1595d`) and `f96381d` "ignored mutants". A fast-forward pull brings it current.

---

## Executive summary

**Status: promising early-WIP prototype, dormant since ~Sept 2025.** The foundation — card/effect data model, poker-hand detection with Balatro-specific hands, hand leveling, and joker chips/mult scoring — is real, working, and reasonably tested (383 tests green with `--features funky`). But the headline scoring pipeline is incomplete (3 of 4 board scoring phases are `todo!()` and panic), only ~15 of 69 effect types actually score, entire Balatro subsystems are absent (spectrals, vouchers, editions, seals, shop/economy, blinds), and neither of the branch's two stated goals — a solver, or dynamic custom mods — is reachable yet. Quality is prototype-grade: debug `println!`s in scoring paths, an unused dependency, a stub example, zero tests on the two largest files, and CI that mostly doesn't see the code.

**Overall grade: C+ / "solid skeleton, unfinished muscles."** The architectural bets are good; the work stopped mid-stride.

---

## Timeline & activity

| Period | Activity |
|---|---|
| 2025-03 → 2025-04 | Main build-out: 150 commits (BuffoonCard, decks, jokers, planets, hand detection, scoring phases) |
| 2025-09 | Short burst: 9 commits (capacity, half joker, phase-4 joker scoring) |
| 2026-05 | 2 commits: merge of main's 0.7.0 no_std release into funky; mutants config |
| 2026-05 → now | Dormant (~8 months since substantive work) |

161 commits by two author identities (folkengine 97, ElectronicPanopticon 64). Commit hygiene is rough — many messages are `...`, single words ("capacity", "board"), or typos ("baffoon", "fucky" — the typos do **not** appear in current source, only in messages).

## Size & shape

- **5,919 insertions across 31 files** vs main; the funky module is **5,778 lines across 17 files**.
- Layered on the core cleanly: `BuffoonCard` embeds core `Pip`s; `BuffoonPile` implements the core `Ranged` trait (`buffoon_pile.rs:460`) so combinatorics/hand analysis delegate to the mature `basic` engine. Only ~50 lines of core were touched (small helpers).
- Feature-gated: `funky = ["std", "dep:phf"]`, off by default, `std`-required — deliberately outside the new no_std core. `cargo build --no-default-features` stays green.

## Verification results (this evaluation, at `cc1595d`)

| Check | Result |
|---|---|
| `cargo test --all` (default features) | ✅ 304 unit + 10 wasm-adjacent + 99 doctests pass — **but compiles zero funky code** |
| `cargo test --features funky` | ✅ 383 unit tests pass (≈79 funky tests + 20 rstest cases), 100 doctests |
| `cargo clippy --features funky` (default level) | ⚠️ **6 lib warnings, all in funky** (`unwrap()` on Option `buffoon_card.rs:70`, collapsible ifs, `sort_by_key`) + 165 in test code |
| `cargo build --no-default-features` | ✅ clean (funky correctly isolated) |
| `cargo fmt --all -- --check` | ✅ clean |
| CI coverage of funky | ⚠️ **Partial-to-none**: the test matrix runs `cargo test --all` (default features) and the clippy job runs default features — neither compiles funky. Only the doc, wasm-build, and llvm-cov `--all-features` jobs touch it. A funky regression would pass the primary CI gates. |

## What works today (solver-relevant capability)

A caller enabling `--features funky` can, right now:

1. Build decks: standard 52, Abandoned (40), Checkered (`Deck::basic_buffoon_pile()` etc.).
2. Construct hands with `bcards!("AS KD QC JS TH")`.
3. Detect any Balatro hand type, including FiveOfAKind / FlushHouse / FlushFive and 4-card straights (`determine_hand_type`, `buffoon_pile.rs:133`).
4. Track and level poker hands with correct Balatro base chips/mult; apply planet cards (`PokerHands::increment`).
5. Score **phase-4 joker contributions** for a played hand (`BuffoonBoard::scoring_phase4_joker_scoring`, proven by 4 end-to-end tests in `board.rs`).

What it **cannot** do: run a full scoring pipeline (phases 1–3 `todo!()` → panic), apply enhancements/editions/seals, use ~52 of ~95 declared jokers (effects still `MPip::Blank`) or ~54 of 69 effect variants (silent `_ => 0` fallthrough), model shop/economy/blinds, or accept externally defined effects (the "custom mods" goal — all effect interpretation is hard-coded `match` arms; the `phf` dependency presumably intended for a static registry is entirely unused).

## Quality evaluation

**Strengths**

- **Right architecture for the goal.** Cards as data (`const` BuffoonCards) + effects as data (`MPip` enum) + a thin interpreter is exactly the shape a solver and mod system want; reusing the core `Ranged` engine for hand analysis avoided reimplementing combinatorics and inherits its test maturity.
- **The hard algorithmic part is done and tested.** Hand detection (incl. Balatro-only hands and 4-card straights for Four Fingers/Shortcut) and hand leveling match Balatro's tables; `buffoon_pile.rs` carries 37 tests + 20 rstest cases.
- **Scoring math that exists is verified** — chips/mult per-card and per-pile calculators have targeted tests (e.g. Odd Todd/Even Steven, Half Joker).
- Consistent naming in code (`BuffoonCard`/`BuffoonPile`/`BuffoonBoard`); clean feature isolation from the no_std core.

**Weaknesses**

- **Test coverage is inverted relative to risk in the data files:** `joker.rs` (1,459 lines — the largest file) and `decks/basic.rs` (620) have **zero tests**; a typo'd joker cost or wrong deck composition would go unnoticed. Overall: 81 `#[test]` fns for 5,778 lines, concentrated in two files.
- **Runtime landmines:** 3 `todo!()` in `board.rs:31-41`; silent zero-scoring for unhandled `MPip`s (a wired joker like `MultPlusOn5Ranks` currently scores 0 with no error).
- **WIP debris:** 6 `print!`/`println!` in production scoring paths (`buffoon_card.rs:186-220`); jokers 96–150 exist only as commented-out lines (`joker.rs:1393-1458`); a joker literally named `HACK` (`joker.rs:541`); "TODO: HACKY" on the central hand-detection fn; unused `phf` dependency; `examples/buffoon.rs` is a 23-line stub that never scores.
- **Docs are journal, not reference:** first-person "DIARY"/"STORY TIME" narratives (e.g. `fpips.rs:38-68`), profanity in doc comments — charming in a devlog, wrong for a published crate.
- **API leaks:** `BuffoonBoard::new` takes a `Draws` whose module is private — external callers can't name the type; `ToggleCard` is likewise unexported.
- **CI blind spot** (see table above) — the most consequential process issue: funky code can rot without CI noticing, and the 0.7.0-merge compatibility was luck, not enforcement.

## Risk register

| Risk | Severity | Note |
|---|---|---|
| CI doesn't gate funky | High | Regressions invisible until someone builds `--features funky` locally |
| `todo!()` panics in public API | High | Any consumer calling phases 1–3 crashes |
| Silent `_ => 0` effect fallthrough | Medium | Produces plausible-but-wrong scores — worst failure mode for a solver |
| Untested 2,000+ lines of card data | Medium | Data errors (costs, effects) undetectable |
| Closed effect interpreter vs "custom mods" goal | Medium | Reaching the goal likely requires a breaking redesign of `MPip` handling |
| 8-month dormancy | Low | Knowledge decay; the merge kept it compiling |

## Recommended next steps (priority order)

1. **Add funky to CI** — `cargo test --features funky` in the test matrix and clippy with the feature on. Cheapest, highest-value fix; do it before touching code.
2. **Kill the landmines:** remove scoring-path `println!`s; fix the 6 clippy warnings; replace `todo!()` phases with `unimplemented!`-documented errors or implement them (phase 1 — played-card chips — is the natural next milestone and unblocks a minimal end-to-end score).
3. **Make the example real:** extend `examples/buffoon.rs` to deal → select → detect → score with a couple of jokers; it becomes both living documentation and a smoke test of the solver path.
4. **Data tests for `joker.rs`/`basic.rs`:** cheap invariant sweeps (unique names, cost > 0, deck sizes, every non-Blank `MPip` variant reachable in `calculate_*`) catch the data-file risk.
5. **Decide the mod-extensibility design** before wiring the remaining ~107 jokers — if the `MPip` match-interpreter is to be replaced by a registry/trait boundary (the unused `phf` hints at this), do it while only ~15 effect kinds are implemented, not after 69.
6. Fast-forward local `funky` to `origin/funky` (20 commits) before resuming work.

## Relationship to EPIC-02 (Ganjifa)

None technically — Ganjifa builds on the `basic` no_std core; funky is a std-only superstructure. They only compete for maintainer attention. The funky branch predates the epics convention, hence its retro-numbering as EPIC-01; its story-level breakdown with current checkbox state lives in [`EPIC-01_Funky.md`](EPIC-01_Funky.md).
