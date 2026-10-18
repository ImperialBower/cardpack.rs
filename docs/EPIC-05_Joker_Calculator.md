# EPIC-05: cardpack.rs ↔ Balatro Joker Calculator integration

Oct 18, 2026 · @Christoph · *Updated 2026-10-18: facts reconciled against the code on branch `epic-05`; Milestone 0 (prerequisites) added.*

## Summary

The Joker Calculator stops carrying its own scoring math and becomes a thin display over cardpack.rs's `funky` engine. Every number in the breakdown table comes from `BuffoonBoard`. Every joker card is a real `BuffoonCard` from `funky::decks::joker`. The "updatable" fields edit the card's `MPip` before the board re-scores.

This is the first consumer of the solver seam that EPIC-01 Story 8 built for exactly this purpose. The engine does not yet expose the one thing the calculator is for: a step-by-step trace of how chips and mult moved. So the keystone story adds a `ScoreTrace` to the fold pipeline. Everything else is packaging: a wasm boundary, a JSON shape the Design artifact can render, and a joker catalog with display names.

The design bundle (Design artifact pages, architecture diagram) lives in [`docs/design/balatro-joker-calculator/`](design/balatro-joker-calculator/README.md). The calculator front end is not in this repo.

## Status

As of 2026-10-18, branch `epic-05` at `d042bd8`. Milestone 0 code landed in `c762538`.

| Component | Status |
| --- | --- |
| Milestone 0 — prerequisites landed (clippy, public joker list, `ScoreOp` serde, strict card parsing, serde round-trip tests) | **Complete** (`c762538`) |
| Milestone 0 — open decisions (`Score` type, `board.rs` split, adapter placement) | Planned |
| Story 1 — `ScoreTrace` | Planned |
| Story 2 — joker catalog | Planned (list + blank reasons done in Milestone 0; names not started) |
| Story 3 — `BoardSpec` | Planned |
| Stories 4–5 — `funky-wasm`, golden fixtures | Planned |
| Stories 6–8 — calculator renders the kernel | Planned |
| Milestone D — polish | Planned |

## Current state

The two sides already agree on the model: chips × mult, then base hand, played cards, held cards, and jokers left to right. Only one side is correct, and it is not the calculator.

|  | cardpack.rs `funky` (v0.11.1, `--features funky`) | Joker Calculator (Design artifact, today) |
| --- | --- | --- |
| Joker model | **112** `BuffoonCard` consts, all listed in `Joker::ALL_JOKERS` (`src/funky/decks/joker.rs:166`). The effect is data (`enhancement: MPip`, 142 variants, `src/funky/types/mpip.rs`). 8 jokers stay `Blank` with a reason in `Joker::BLANK_WITH_REASON` (`joker.rs:306`). Some non-`Blank` jokers still score 0 because they are run-state jokers, e.g. Supernova (`MultPlusOnHandPlays`) and Loyalty Card (`MultTimesEveryXHands`), listed as non-scoring at `joker.rs:2675,2677` | 28 hand-typed presets. Each is reduced to one of `+Chips` / `+Mult` / `×Mult` and a number |
| Scoring | `BuffoonBoard::score()` (`src/funky/types/board.rs:914`) runs four phases through `fold_played_cards` (`:342`), `fold_held_cards` (`:478`) and `fold_jokers` (`:556`). **One item can apply several `ScoreOp`s.** A played card applies 3 per pass: built-in, then special (Lucky / Glass / Custom), then edition (`:359-392`). Retriggers repeat all 3 (`:356`). A held card applies one op `1 + retriggers` times (`:485-494`). A joker applies 2: its effect, then its edition (`:564-584`) | JS in the artboard: base → card chips → jokers. No held-card phase, no editions, no retriggers |
| Conditions | Evaluated from the board. `builtin_joker_op` (`board.rs:591`), `joker_x_mult` (`:776`) and `counter_joker_op` (`:3096`) read `played`, `in_hand`, `jokers`, `deck`, money, discards and blind | An "active" checkbox the user ticks by hand |
| Randomness | `score()` is the deterministic floor: with no RNG, Lucky and Misprint resolve to `ScoreOp::Nothing` (`board.rs:366,568`). `score_with_seed(u64)` (`:931`) rolls them **inside** op resolution | Not modelled |
| Mult arithmetic | `Score { chips: usize, mult: usize }` (`src/funky/types/score.rs:8`). `multi_mult(f32)` **ceils** to `usize` (`score.rs:43-47`). `Score::score()` is a plain `chips * mult` (`score.rs:53`) with no overflow handling, and `usize` is **32 bits on wasm32** | Fractional mult kept (`×1.25` shows as 1.25) |
| Step-by-step trace | **None.** The folds return only the final `Score` | The whole point of the UI: a row per source with running chips, mult and score |
| Serde | `Score`, `MPip`, `BuffoonCard`, `BuffoonPile`, `Edition`, `HandType` derive serde. `ScoreOp` does too since Milestone 0 (`src/funky/types/effect.rs:25`). The JSON of `ScoreOp`, `HandType` and `Edition` is pinned by format tests | — |
| Card parsing | `BuffoonCard::from_str` rejects unknown indexes since Milestone 0 (`src/funky/types/buffoon_card.rs:373`). `"__"` is the explicit blank card (`:372`) | — |
| wasm | Compiles on `wasm32-unknown-unknown` for every feature combo ([docs/wasm.md](wasm.md); CI `.github/workflows/CI.yaml:121`). CI's wasm **test** job runs only `--features full --test wasm` (`CI.yaml:147`), so funky never runs on wasm. `wasm-bindgen-test` is already a dev-dependency. No bindings crate yet | Browser-only. No way to call Rust |
| Display names | **No joker names anywhere.** Jokers are known by const name and `Pip` index/symbol. There is no `fn name()` on `BuffoonCard`, and no `.ftl` file under `src/localization/` names a joker | Hard-coded English names |

Sources: `src/funky/types/{board,score,effect,mpip,buffoon_card}.rs`, `src/funky/decks/joker.rs`, `docs/EPIC-01_Funky.md` (Story 8 "solver enablement"), `docs/EPIC-01a_Joker_Wiring_Backlog.md`, `.okf/architecture/funky-engine.md`.

## Goals and non-goals

**Goals**

- **One source of truth**: a number shown in the calculator is a number `funky` produced. The JS keeps no scoring arithmetic.
- **Every breakdown row is explainable**: it names the card, the part that fired (effect, special, or edition), the `ScoreOp` it produced, and the running `Score` after it.
- **Editable effects stay inside the type system**: a user edit becomes a `BuffoonCard` with a changed `enhancement` (or `edition`), not an override applied after the fact.
- **Conditions resolve from the board**, not a checkbox: the calculator shows *why* Jolly Joker did or did not fire because the kernel reports it.
- **The kernel stays pure**: no DOM, no JS types, no calculator-specific code in `src/funky/`. The boundary is a separate crate (`funky-wasm` or similar), a delivery adapter in the domain-kernel sense.
- **Parity test**: the kernel trace and the rendered table agree on every golden board.

**Non-goals**

- Playing Balatro. No shop, no antes, no run loop in the calculator. `BuffoonBoard` already has them; the calculator ignores them.
- Pixel-accurate joker art. The simplified card stays simplified.
- Modded jokers (`MPip::Custom` / `EffectRegistry`) in v1. The seam exists; wiring it into the UI is a follow-on.
- Filling the 8 `Blank` jokers. They render as "not modelled" with their stated reason.
- A WIT / component-model boundary. The domain-kernel assessment explicitly does not recommend it (`.okf/architecture/domain-kernel.md`).

## Architecture

Three layers, one direction of truth: the UI describes a board, the kernel scores it and says how, the adapter only translates.

```mermaid
flowchart LR
    UI["Joker Calculator (UI)<br/>joker cards + breakdown table<br/>edits become BoardSpec JSON<br/>no scoring arithmetic"]
    AD["funky-wasm adapter<br/>wasm-bindgen + serde_json<br/>BoardSpec → BuffoonBoard<br/>ScoreTrace → JSON"]
    K["cardpack::funky (kernel)<br/>BuffoonBoard::score_traced()<br/>folds emit one ScoreStep per op<br/>pure: no I/O, no JS types"]
    CAT["joker-catalog.json<br/>built from Joker::ALL_JOKERS:<br/>name, rarity, cost, MPip, Blank reason"]
    UI -- BoardSpec --> AD
    AD -- "ScoreTrace (JSON)" --> UI
    AD -- BuffoonBoard --> K
    K -- ScoreTrace --> AD
    K -. build step .-> CAT
    CAT --> UI
```

*Figure: integration architecture: 3 layers, 1 generated file (also in [`design/balatro-joker-calculator/docs/architecture.svg`](design/balatro-joker-calculator/docs/architecture.svg)).*

The UI never computes. It sends a `BoardSpec` and renders the `ScoreTrace` that comes back. The joker catalog is generated from `Joker::ALL_JOKERS`, the same list the joker tests guard, so the picker cannot drift from what the engine scores.

**The two new kernel types** (in `src/funky/types/trace.rs`, behind `funky`):

```rust
pub struct ScoreStep {
    pub phase: Phase,            // Base, Played, Held, Joker
    pub source: StepSource,      // HandType, Card(BuffoonCard, index), Joker(BuffoonCard, index)
    pub part: StepPart,          // Effect, Special, Edition
    pub retrigger: u8,           // 0 = first pass, 1.. = each retrigger
    pub op: ScoreOp,             // what it produced (Nothing when a condition fails)
    pub fired: bool,             // op != Nothing
    pub rolled: bool,            // the op came from an RNG roll (Lucky, Misprint)
    pub after: Score,            // running score once applied
}

pub struct ScoreTrace {
    pub hand_type: HandType,
    pub steps: Vec<ScoreStep>,
    pub total: Score,
}
```

**Why one step per op, not one per item.** The first draft said each item resolves to one `ScoreOp`. The code does not work that way. A played card applies up to 3 ops per pass and a joker applies 2 (see Current state). Folding them into one `Seq` step would hide the edition, and the edition is a row the calculator must show. So the trace records every `apply` call. The card's `MPip` is not a separate field, because `source` already carries the `BuffoonCard`.

The folds already call `op.apply(score)` at every point that changes the score. The trace is a sink those calls push into, so `score()` becomes `score_traced().total` and no scoring logic moves. A failed condition is a step with `op: Nothing, fired: false`, which is the "skipped" row the calculator shows today.

**The boundary** (`funky-wasm`, a new crate; placement is decision 0h, open question 2):

- `score(spec_json: &str) -> String`: `BoardSpec` in, `ScoreTrace` out, both serde.
- `catalog() -> String`: the generated joker catalog.
- `BoardSpec` names jokers by their const identifier (`"JOLLY_JOKER"`) plus an optional `enhancement` override as an `MPip` and an optional `edition`. Played and held cards are card indexes (`"AS"`), parsed with the strict `BuffoonCard::from_str`. Hand levels are optional, default 1.
- No `BuffoonBoard` field is exposed raw. The adapter is the only place `wasm_bindgen` or `serde_json` appear.

This follows the domain-kernel split already used in the repo (`audit-2026-07-18-domain-kernel.md`): the kernel is pure, the adapter owns the format.

## Stories

Ordered so each milestone ships something usable on its own. Milestone 0 clears the ground. Story 1 is the keystone; nothing downstream can be honest without it.

### Milestone 0 — prerequisites (cardpack.rs)

Found by checking this EPIC's first draft against the code on 2026-10-18.

**Landed** (branch `epic-05`, commit `c762538`, 2026-10-18):

- [x] **0a. Clippy green on Rust 1.99.** A test in `src/basic/decks/registry.rs` tripped the new `needless_collect` lint, so `cargo clippy --all-targets -- -D warnings` failed.
- [x] **0b. Public joker list.** `Joker::ALL_JOKERS` / `ALL_JOKERS_SIZE` (`src/funky/decks/joker.rs:166`), `Joker::pile_all()` (`:282`), `Joker::BLANK_WITH_REASON` (`:306`) and `Joker::blank_reason()` (`:355`) moved out of `#[cfg(test)]`. Tests: `pile_all__holds_every_joker_exactly_once`, `blank_reason__explains_a_blank_joker`, `blank_reason__is_none_for_a_wired_joker`.
- [x] **0c. `ScoreOp` derives serde** (`src/funky/types/effect.rs:25`). Tests: `score_op__serde_round_trip`, `score_op__serde_format` (pins the JSON).
- [x] **0d. Strict card parsing.** `BuffoonCard::from_str` returned a blank card for *any* unknown index, so a typo in a test or fixture added a fake blank card to the hand. Unknown indexes are now `CardError::InvalidIndex` (`src/funky/types/buffoon_card.rs:373`); `"__"` is still the blank card. `BuffoonPile::forgiving_from_str` (`src/funky/types/buffoon_pile.rs:342`) now logs a warning, like `Pile::forgiving_from_str`. A probe that made `bcards!` strict found no typo in the existing 158 call sites. Test: `from_str__unknown_index_is_an_error`.
- [x] **0e. Serde round-trip tests** for every joker, `BuffoonPile`, `HandType` and `Edition`, plus format tests that pin the JSON of `HandType` and `Edition`. These pin the shape `BoardSpec` and `ScoreTrace` will carry.

**Open:**

- [ ] **0f. Decide the `Score` type** (open question 1). If it changes, it changes here, before Story 1, in its own commit.
- [ ] **0g. Decide whether to move the scoring folds out of `board.rs`** (8,342 lines; tests start at `:3180`). Story 1 edits `:342`, `:478` and `:556`. A move after Story 1 would mix a refactor into the trace diff.
- [ ] **0h. Decide `funky-wasm` placement** (open question 2).
- [ ] **0i. Fix the half-stale comment at `board.rs:534-536`.** It says `MultTimesOnEmptyJokerSlots` (Joker Stencil) and `MultTimesEveryXHands` (Loyalty Card) are not applied. Stencil is applied now; Loyalty Card still is not.
- [ ] **0j. Backfill `CHANGELOG.md` for 0.11.1.** The tag exists; the entry does not. Do it before this EPIC adds entries.
- [ ] **0k. Run funky on wasm in CI.** Add a funky case to `tests/wasm.rs` and `funky` to the wasm-test job's features (`CI.yaml:147`). Story 4's browser test then has a known-good base.

### Milestone A — the kernel can explain itself (cardpack.rs)

**Story 1: `ScoreTrace` in the fold pipeline**

- [ ] Add `src/funky/types/trace.rs` with `ScoreStep`, `ScoreTrace`, `Phase`, `StepSource`, `StepPart` (all `Clone + Debug + Serialize + Deserialize`, consistent with the rest of `funky/types`)
- [ ] Thread an `Option<&mut ScoreTrace>` sink through `fold_played_cards`, `fold_held_cards` and `fold_jokers`. It is a new parameter: `fold_held_cards` takes no RNG today (`board.rs:478`), so there is no existing parameter to copy on all three
- [ ] Push one step at every `apply` call: 3 per played-card pass (`board.rs:359,386,392`), one per held-card application (`:492`), 2 per joker (`:578,583`). Phase 1 pushes one `Base` step carrying the `HandType` and level
- [ ] Mark RNG-resolved ops (Lucky `:366`, Misprint `:568`) with `rolled: true`
- [ ] `BuffoonBoard::score_traced()`, `score_traced_with_seed(u64)`, `score_traced_with_registry(&EffectRegistry)`: the same trio as today's `score*` (`board.rs:914,931,988`). No current entry point takes both an RNG and a registry; this story does not add one
- [ ] Invariant test: for every golden board, `score_traced().total == score()` and `steps.last().after == total`
- [ ] Invariant test: folding the steps' ops from the base `Score` reproduces each `after`
- [ ] `examples/buffoon.rs` prints the trace instead of (or alongside) the four phase totals

*Acceptance:* a Flush with Joker + Droll Joker + Crafty Joker + Cavendish yields a trace whose joker effect rows read `+4 Mult`, `+10 Mult`, `+80 Chips`, `×3 Mult` in that order. A Pair with Droll Joker yields a `fired: false` row for it.

**Story 2: joker catalog with display names**

- [x] All jokers in a stable order: `Joker::ALL_JOKERS` / `Joker::pile_all()` (Milestone 0b)
- [x] Blank reasons: `Joker::blank_reason()` (Milestone 0b)
- [x] Test: every joker appears exactly once (`pile_all__holds_every_joker_exactly_once`), and every `Blank` joker carries a non-empty reason (`all_jokers__every_blank_joker_has_a_stated_reason`, pre-existing)
- [ ] A `JokerInfo { ident, name, rarity, cost, enhancement, blank_reason }` accessor over `Joker::ALL_JOKERS`
- [ ] **A name for each of the 112 jokers.** No `.ftl` file has joker names, so this is a full table, not an override table. Decide where it lives: a `.ftl` (behind `i18n`) or a plain `const` table in `funky` (no feature cost)
- [ ] `ident` (`"JOLLY_JOKER"`) needs a const-name ↔ card map. There is none today; build it next to `ALL_JOKERS` and test it covers all 112

**Story 3: `BoardSpec`, a serde-friendly way to build a `BuffoonBoard`**

- [ ] `BoardSpec { hand_levels, played, held, jokers: Vec<JokerSpec>, money, discards_left, hands_left, blind }` with every field optional and defaulted
- [ ] `JokerSpec { ident, enhancement: Option<MPip>, edition: Option<Edition>, debuffed: bool }`: the override is the "updatable effect"
- [ ] `TryFrom<BoardSpec> for BuffoonBoard` with a typed error naming the bad field (unknown ident, unknown card index, 6 jokers in 5 slots, more than 5 played cards)
- [ ] Round-trip test: `BoardSpec` → board → trace → JSON → parse, for the golden boards

*Milestone A done when:* `cargo test --features funky` is green and `cargo run --example buffoon --features funky` prints a trace.

### Milestone B — the boundary (new crate `funky-wasm`)

**Story 4: wasm adapter**

- [ ] New crate depending on `cardpack = { default-features = false, features = ["funky"] }` plus `wasm-bindgen`, `serde-wasm-bindgen` or `serde_json`
- [ ] `.cargo/config.toml` with the `getrandom_backend="wasm_js"` cfg from `docs/wasm.md`
- [ ] Exports: `score(spec: &str) -> Result<String, JsError>`, `catalog() -> String`, `version() -> String`
- [ ] `wasm-bindgen-test` runs the golden boards in a headless browser in CI (`wasm-pack test --headless --chrome`)
- [ ] Size budget recorded in the README after `wasm-opt -Oz`; `i18n` and `colored-display` off
- [ ] If the crate is a workspace member: prove `cargo deny check bans` and the CI `kernel-purity` job still pass (see open question 2)

**Story 5: golden fixtures shared across the boundary**

- [ ] `fixtures/golden/*.json`: 10–15 `BoardSpec` + expected `ScoreTrace` pairs, checked into cardpack.rs and consumed verbatim by both the Rust tests and the UI tests
- [ ] At least one fixture per `ScoreOp` variant, one with a failed condition, one with a retrigger, one with an edition, one with a `Blank` joker
- [ ] Note: `cargo package` ships only `include = [...]` paths (`Cargo.toml`), so fixtures under `fixtures/` stay out of the published crate. That is fine for tests; say so in the fixture README

### Milestone C — the calculator renders the kernel (Design artifact / web)

**Story 6: replace the JS math with the wasm call**

- [ ] Load `funky_wasm.js` + `.wasm`. On any edit, build a `BoardSpec` from UI state and call `score`
- [ ] Delete `HANDS`, `JOKERS` and the step loop from the artboard. The breakdown table maps 1:1 onto `ScoreTrace.steps`
- [ ] Joker picker populated from `catalog()`. The card's note shows the real `MPip` and, for `Blank`, the reason
- [ ] The "active" checkbox becomes read-only `fired` state computed by the kernel. A manual override remains only as the `debuffed` flag
- [ ] Editing the value or effect type writes a `JokerSpec.enhancement` override and shows an "overridden" badge on the card

**Story 7: played and held cards**

- [ ] A hand editor (rank + suit per card, up to 5 played, up to 8 held) replaces the single "played card chips" number
- [ ] Hand type comes from the trace, not a dropdown. The dropdown becomes a hand-level editor per `HandType`
- [ ] Group trace rows by card (one card can make 3 steps per pass, times its retriggers)

**Story 8: parity test**

- [ ] A browser test renders each golden fixture and asserts the table's cells equal the fixture's trace

### Milestone D — polish

- [ ] Seeded mode: a seed field switches to `score_traced_with_seed`. The trace marks rolled steps
- [ ] Edition picker (Foil / Holo / Poly) per joker, rendered as a corner mark on the simplified card
- [ ] Export: copy the current `BoardSpec` JSON; paste one to restore a board
- [ ] Custom effects: a `MPip::Custom(id)` entry whose `ScoreOp` the user types, routed through an `EffectRegistry` in the adapter

## Open questions and risks

1. **`Score` precision and width.** Two problems in one type (`src/funky/types/score.rs`):
   - `mult` is `usize` and `multi_mult` ceils after each ×mult (`:43-47`). Hologram at ×1.25 on mult 4 gives 5, and two ×1.5 jokers round twice. Balatro keeps fractional mult. The calculator keeps decimals.
   - `chips * mult` is unchecked (`:53`), and `usize` is 32 bits on wasm32. Late-game Balatro scores pass 4.29 × 10⁹, so the calculator would overflow in the browser.
   - Any fix is a breaking change to `Score` and to the pinned `ScoreOp` JSON (`Add(Score)`). It belongs in Milestone 0f, before Story 1.
2. **Crate placement.** `funky-wasm` inside cardpack.rs as a workspace member (shared CI, lock-step versions) or a sibling repo (the kernel's dependency tree is untouched)? cardpack has no `[workspace]` today. Known trap: a self dev-dependency that enabled features poisoned cargo-deny's feature-unified resolve, and no cargo-deny setting could undo it (`.okf/decisions/examples-flag-free-alias.md`, comment in `Cargo.toml`). A workspace member that turns on `funky` plus `wasm-bindgen` may hit the same wall in the `kernel-purity` job. The domain-kernel audit argues for a sibling repo.
3. **Where the calculator lives.** The Design artifact cannot load a `.wasm` from the canvas page today, so Milestone C needs a real web host (a static page in the repo, `examples/wasm.rs` style). The artboard stays as the design reference.
4. **`funky` requires `std`.** Fine on `wasm32-unknown-unknown`, but it rules out a `no_std` component later. Acceptable for this epic.
5. **Display names.** No joker names exist in the i18n resources (Story 2). The const name title-cased is wrong for several ("Dna", "Oops All 6s"), so a hand-checked table of all 112 is needed.
6. **Trace size.** One played card makes up to 3 steps per pass. Retriggers on a 5-card hand with Hack + Sock and Buskin can produce 40+ steps. The table needs grouping by card (Story 7).
7. **Every played card scores.** The engine has no scoring-card vs kicker split (`src/funky/types/mpip.rs:60-61`). In Balatro, a kicker in a Pair does not add chips. The calculator will show kicker rows that Balatro would not. Fix in the engine (its own story) or show it as a known gap.
8. **No Ace-low straights** (`src/funky/types/buffoon_pile.rs:472-474`). `A-2-3-4-5` is not a straight here. The hand editor in Story 7 makes this easy to hit.

**Risks:** scope creep toward a playable game (the board already has a shop; the calculator must keep saying no); silent drift between catalog and engine if Story 2 is hand-maintained instead of generated from `Joker::ALL_JOKERS`.

## Definition of done

- The breakdown table in the calculator is produced entirely from a `ScoreTrace` returned by `cardpack::funky` via `funky-wasm`. The artboard contains no scoring arithmetic.
- Every one of the 112 jokers can be placed in a slot. 104 resolve to an effect; 8 show their `Blank` reason.
- Editing a joker's effect or edition changes the trace through a `JokerSpec` override, visibly marked as an override.
- Conditions are reported by the kernel (`fired`), not toggled by the user.
- The golden fixtures pass in `cargo test --features funky`, in `wasm-bindgen-test`, and in the browser parity test.
- This EPIC closes with a status table in the EPIC-01a style, and `CHANGELOG.md` lists `ScoreTrace`, `BoardSpec` and the catalog accessor under Added.

## Verification

```bash
cargo fmt --check
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo test --all-features
cargo build --no-default-features
cargo run --example buffoon --features funky
cargo test --target wasm32-unknown-unknown --features full,funky --test wasm   # after 0k
```

Milestone 0 (landed part) was checked on 2026-10-18 with the first six commands: all pass, 1011 lib tests with `--all-features`.
