# EPIC-06: Funky Extraction (FX)

Oct 18, 2026 · @Christoph

## Summary

- **Builds:** `buffoonery`, a standalone repo and crate for the Balatro-style engine, and a cardpack without it.
- **Why:** the engine is becoming the foundation of a solver (EPIC-05), with a WIT boundary and YAML configuration. Cardpack is a card *value* kernel with no game state, and it should not carry a game engine.
- **Shape:** move `src/funky/` (18,120 lines) into a new repo that depends on cardpack. Cardpack deprecates its `funky` feature in 0.11.2 and removes it in 0.12.0.
- **Proves it:** the engine's 477 lib tests and 3 doctests pass in `buffoonery` against published cardpack, and cardpack's gates pass with no `funky` feature.
- **Status:** In progress. Phases 0–2 done: `buffoonery` 0.0.1 and cardpack 0.11.2 (the deprecation) are on crates.io (2026-10-19). Next: Phase 3, removal in 0.12.0.

---

## Context

`funky` is an off-by-default cardpack feature (`Cargo.toml:36`, `funky = ["std", "serde"]`). It is the only cardpack code that models a *game*: boards, money, rounds, shop, scoring (`.okf/architecture/funky-engine.md`). The rest of cardpack holds cards, piles and decks, with no game state (`.okf/architecture/domain-kernel.md:10-15`).

The cut is clean:

- `src/funky/` uses only **public** cardpack items: `BasicCard`, `BasicPile`, `CardError`, `FrenchRank`, `FrenchSuit`, `Pip`, `PipType`, `Ranged`, `TarotRank`, `TarotSuit` (all via `crate::prelude`).
- `Pip`'s fields are public with no `#[non_exhaustive]` (`src/basic/types/pips.rs:79-85`), so the joker `const`s still compile outside cardpack.
- Nothing outside `src/funky/` and `src/preludes/` depends on it. `src/lib.rs:360-366` gates both modules on the feature; `src/preludes/` holds only the funky prelude.
- The feature adds no dependency of its own (`std` and `serde` are shared).

Decided on 2026-10-18 while scoping EPIC-05 (its open question 2 / item 0h). The two options were: keep the engine here and wrap it from a new repo, or move it. Moving won, because EPIC-05's keystone changes (`ScoreTrace`, the `Score` type, the fold split) all edit the engine. Kept here, every one of them would need a cardpack release before the solver could use it.

**This EPIC does not** change the engine's behavior, add EPIC-05 features, or touch any non-funky cardpack API. Cardpack cannot re-export the engine afterwards: `buffoonery` depends on cardpack, so a re-export would be a dependency cycle.

---

## Status

| Component | Status |
|---|---|
| Phase 0 — name, scaffold, copy | **Complete** except 0d (2026-10-18; scaffold in `buffoonery` `538cee4`, copy in its working tree) |
| Phase 1 — engine builds and passes in `buffoonery` | **Complete** (2026-10-18; CI green at `buffoonery` `ce9fd8a`; 0.0.1 on crates.io, tag `v0.0.1` at `b7c16bf`) |
| Phase 2 — cardpack 0.11.2 deprecation | **Complete**: 0.11.2 on crates.io (tag `v0.11.2`, merge `25ff170`, 2026-10-19); `buffoonery`'s deny skips removed |
| Phase 3 — cardpack 0.12.0 removal | Planned |
| Phase 4 — docs and `.okf/` | Planned |

---

## Goals

- **One home for the engine.** After 0.12.0, Balatro code exists only in `buffoonery`.
- **No behavior change.** The move is mechanical: the same tests pass with the same results.
- **A traceable start.** `buffoonery` starts fresh, with no cardpack history. Its first commit names the cardpack commit the engine was copied from.
- **A clean warning period.** Users of `cardpack/funky` get one release (0.11.2) that still works and tells them where the engine went.
- **A purer cardpack.** The domain-kernel description becomes simply true: no game state anywhere in the crate.

## Scope

- The new repo depends on a **published** cardpack (`0.11`, `features = ["std", "serde"]`), never on a path or git dependency in its released form.
- The engine moves **as is**, including the EPIC-05 Milestone 0 work (`c762538`). Behavior changes start only after Phase 1 is green.
- Removal is **semver-major** for cardpack (0.12.0). `cargo-semver-checks` (run in the CI Clippy job) will flag it; that is expected.
- EPIC-01's documents stay in cardpack as history, marked "moved".

---

## Design

### The `buffoonery` repo

```text
buffoonery/
├── Cargo.toml          # depends on cardpack = { version = "0.11", features = ["std", "serde"] }
├── src/
│   ├── lib.rs          # was cardpack/src/funky/mod.rs
│   ├── prelude.rs      # was cardpack/src/preludes/funky.rs
│   ├── decks/          # was cardpack/src/funky/decks/
│   ├── types/          # was cardpack/src/funky/types/
│   └── macros.rs       # was cardpack/src/funky/macros.rs
└── examples/           # buffoon.rs, funky_tour.rs
```

The path rewrites are mechanical:

| In cardpack | In `buffoonery` |
|---|---|
| `crate::prelude::X` | `cardpack::prelude::X` |
| `crate::preludes::funky::X` | `crate::prelude::X` |
| `crate::funky::…` | `crate::…` |
| `$crate::funky::decks::…` (in `macros.rs`) | `$crate::decks::…` |
| `cardpack::preludes::funky::*` (examples, doctests) | `buffoonery::prelude::*` |

**Why a top-level `prelude`, not `preludes::funky`.** Cardpack needed `preludes::funky` to keep funky's names apart from its own prelude. The new crate has one prelude.

**The name is `buffoonery`**, for both the repo (`ImperialBower/buffoonery`) and the crate. `funky` (a thread-dispatch crate) and `buffoon` (a Protocol Buffers crate) are taken on crates.io; `buffoonery` was free on 2026-10-18.

### Deprecation (0.11.2)

```rust
// src/lib.rs
#[cfg(feature = "funky")]
#[deprecated(since = "0.11.2", note = "the funky engine moved to the `buffoonery` crate; removed in 0.12.0")]
pub mod funky;
```

`#[deprecated]` on a module does not warn for every use path in every Rust version. Phase 2 checks that the warning fires for a consumer that writes `use cardpack::preludes::funky::*`. If it does not, the README and CHANGELOG carry the notice, and `preludes` gets the attribute too.

---

## Work Items

### Phase 0 — Name, scaffold, copy

- [x] **0a. Name: `buffoonery`**, repo and crate (decided 2026-10-18). `cargo info buffoonery` found no crate; `ImperialBower/buffoon` and `buffoonery` are not GitHub repos yet.
- [x] **0b. Scaffold the repo.** Create `~/src/github.com/ImperialBower/buffoonery` with the `dev-playbook` skill (CI, `deny.toml`, lint config, `Cargo.toml`, README, `CHANGELOG.md`). The repo starts fresh: no cardpack git history comes along (decided 2026-10-18). The engine's history stays readable in cardpack, up to its last commit before Phase 3.
- [x] **0c. Copy the engine in.** `src/funky/` → `src/` (with `mod.rs` → `lib.rs`), `src/preludes/funky.rs` → `src/prelude.rs`, `examples/buffoon.rs` and `examples/funky_tour.rs` → `examples/`. Record the cardpack commit the copy came from in the first commit message and in `CHANGELOG.md`, so a reader can find the older history.
- [x] **0d.** Backfill cardpack's `CHANGELOG.md` for 0.11.1 (tag exists, entry does not), so 0.11.2's entry has a clean base. Done: 0.11.1 changed only the README (`git diff --stat v0.11.0 v0.11.1`); entry and compare links added.

### Phase 1 — The engine builds and passes in `buffoonery`

- [x] **1a.** Apply the path rewrites from the Design table. Green check: `cargo build`.
- [x] **1b.** Move the funky test helpers that live in cardpack's dev-dependencies (`rstest`, `serde_json`) into `buffoonery`'s `[dev-dependencies]`. Done: the engine needed `cardpack` (`std`, `serde`), `serde`, `rand 0.10` and `log`; its tests and examples need `rstest`, `serde_json` and `env_logger`.
- [x] **1c.** Green check: `cargo test` shows **477 lib tests + 3 doctests** passing (the counts in cardpack at `c762538`: `cargo test --features funky --lib funky::` and `--doc funky`). Done: 477 lib tests pass; 4 doctests (the 3 engine doctests plus a new crate-level example), on stable and on 1.85.0.
- [x] **1d.** Both examples run: `cargo run --example buffoon` and `cargo run --example funky_tour`. `buffoon` must still score **11254**, the value recorded in the 2026-10-18 EPIC-01 evaluation. Done: `buffoon` prints 11254; `funky_tour` exits 0.
- [x] **1e.** CI: fmt, clippy pedantic, test, doc with `-D warnings`, MSRV 1.85, `cargo deny`. Done: CI and Security green on GitHub at `buffoonery` `ce9fd8a` (2026-10-18). Two first-run failures: `bin/security-scan` was never committed because the maintainer's global gitignore excludes `bin` (fixed with `!/bin/` in `buffoonery/.gitignore`), and cargo-deny flagged duplicate `getrandom` 0.3/0.4 (skipped in `buffoonery/deny.toml`; see 2f).
- [x] **1f.** Publish 0.1.0, so cardpack's deprecation note can name a real crate. Done: published as **0.0.1**, not 0.1.0 (2026-10-18, tag `v0.0.1` at `buffoonery` `b7c16bf`). Before publishing, the maintainer raised `buffoonery`'s MSRV from 1.85.0 to **1.99.1**.

### Phase 2 — cardpack 0.11.2 deprecation

- [x] **2a.** `#[deprecated]` on `pub mod funky` (and `pub mod preludes` if needed) per the Design sketch (`src/lib.rs:360-366`). Done: on both modules, and on the exported `bcard!` / `bcards!` macros (`src/funky/macros.rs`), since `#[deprecated]` works on a `#[macro_export]` macro.
- [x] **2b.** Prove the warning fires: a scratch consumer crate with `cardpack = { path = "…", features = ["funky"] }` and `use cardpack::preludes::funky::*;` shows the deprecation warning on `cargo build`. Done: before 2a the scratch consumer built with no warnings; after, it warns on the module, each funky item it uses, and the `bcards!` macro.
- [x] **2c.** `#![allow(deprecated)]` where cardpack's own funky tests and examples would otherwise fail `-D warnings`. Done: in `src/funky/mod.rs`, `src/preludes/mod.rs`, both examples, and `#![cfg_attr(all(test, feature = "funky"), allow(deprecated))]` in `src/lib.rs`, because the test harness names every test from the crate root. `make ayce` green.
- [x] **2d.** README: the Funky section (`README.md:236-269`) and the feature-table row (`README.md:108`) point to the new crate.
- [x] **2e.** `CHANGELOG.md` 0.11.2: a `### Deprecated` entry with the migration line from Compatibility below.
- [x] **2f. Drop the stale `getrandom` 0.3 pin** (`Cargo.toml`, `[target.'cfg(target_arch = "wasm32")'.dependencies]`). Its comment says `rand_core` 0.9 pulls 0.3 in through dev-dependencies, but that comes from `proptest`, which is gated off wasm32, so on wasm nothing else uses 0.3 (`cargo tree -i getrandom@0.3.4 --target wasm32-unknown-unknown` shows only cardpack). Every consumer gets a duplicate `getrandom` from it; `buffoonery` has to skip it in `deny.toml`. Green check: the CI wasm-build and wasm-test commands still pass. Then remove the skips from `buffoonery/deny.toml`. Done in cardpack: all four CI wasm commands pass (3 builds, 5 wasm tests). A scratch copy of `buffoonery` patched to this cardpack has no `getrandom` 0.3 at all, and `cargo deny check bans` passes with the skips removed. After 0.11.2 shipped (2026-10-19), `buffoonery` requires `cardpack = "0.11.2"` and its `deny.toml` has `skip = []`; `cargo deny check` passes.

### Phase 3 — cardpack 0.12.0 removal

- [ ] **3a.** Delete `src/funky/`, `src/preludes/`, and the gated modules at `src/lib.rs:360-366`.
- [ ] **3b.** `Cargo.toml`: remove the `funky` feature (`:36`) and the `buffoon` / `funky_tour` example entries; delete `examples/buffoon.rs`, `examples/funky_tour.rs`.
- [ ] **3c.** CI: remove the funky test step (`.github/workflows/CI.yaml:39-41`) and the funky mention in the clippy comment (`:78`).
- [ ] **3d.** Makefile: remove `test-funky` (`:15`, `:97-103`), drop it from `test` (`:118`), remove the funky MSRV line (`:237`), fix the comments at `:112`, `:117`, `:227`.
- [ ] **3e.** `.cargo/config.toml:23`: drop `funky` from the `cargo ex` alias (and the comment at `:17`).
- [ ] **3f.** README: remove the funky row, section and example lines (`README.md:108`, `:236-269`, `:296`, `:386-387`).
- [ ] **3g.** `docs/TECHNICAL_DEBT.md` and `BACKLOG.md`: move funky items to `buffoonery`; leave a one-line pointer.
- [ ] **3h.** Version 0.12.0. Green check: the Verification block below, with no `funky` anywhere in `git grep -i funky -- src Cargo.toml .github Makefile .cargo`.

### Phase 4 — Docs and `.okf/`

- [ ] **4a.** A "moved" banner at the top of `docs/EPIC-01_Funky.md`, `EPIC-01_Funky_Progress.md`, `EPIC-01a`–`EPIC-01e`, and `docs/presentations/epic-01-funky-balatro-cards.md`.
- [ ] **4b.** `.okf/architecture/funky-engine.md` → a `**Deprecation**` note pointing at `buffoonery` (the OKF rule: mark removed assets, do not delete context).
- [ ] **4c.** `.okf/architecture/feature-flags.md`, `crate-overview.md`, `card-model.md`, `domain-kernel.md`, `workflows/build-and-test.md`, `decisions/examples-flag-free-alias.md`: remove or reword funky mentions.
- [ ] **4d.** `/okf:validate .okf --strict` passes.

---

## Test Plan

- **Engine parity** (Phase 1c): `buffoonery`'s full suite passes unchanged, 477 + 3. Any failure is a move bug, not a behavior change.
- **Golden score** (Phase 1d): `buffoon` prints 11254.
- **Deprecation warns** (Phase 2b): a consumer build shows the warning.
- **Clean removal** (Phase 3h): every cardpack gate passes; `git grep` finds no funky in code, build or CI files.

## Key Files

| File | Role |
|---|---|
| `src/funky/**` | moves out (Phase 1), then is deleted (Phase 3) |
| `src/preludes/funky.rs`, `src/preludes/mod.rs` | become the new crate's `prelude.rs`; deleted here |
| `src/lib.rs:360-366` | deprecated (Phase 2), then removed (Phase 3) |
| `Cargo.toml:36`, example entries | feature and examples removed |
| `.github/workflows/CI.yaml:39-41`, `Makefile`, `.cargo/config.toml:23` | funky build steps removed |
| `README.md`, `CHANGELOG.md` | deprecation notice, then removal |
| `.okf/architecture/funky-engine.md` and related concepts | marked deprecated / reworded |

## Reuse (do NOT recreate)

- The engine itself: copy the files as they are (0c). Do not rewrite them while moving; behavior changes start only after Phase 1 is green.
- `dev-playbook` for `buffoonery`'s scaffold (CI, deny, lint config).
- `.okf/decisions/examples-flag-free-alias.md`: the `cargo ex` alias pattern, if `buffoonery` needs feature-gated examples.

## Compatibility

- **Preserves** every non-funky cardpack API.
- **Deprecates** (0.11.2) `cardpack::funky` and `cardpack::preludes`.
- **Breaks** (0.12.0) the `funky` feature and both modules. Migration:

  ```toml
  # before
  cardpack = { version = "0.11", features = ["funky"] }
  # after
  buffoonery = "0.0.1"
  ```

  and `use cardpack::preludes::funky::*;` → `use buffoonery::prelude::*;`. Engine type names do not change. Two catches: `buffoonery` needs Rust **1.99.1** (cardpack needs 1.85.0), and under Cargo's rules every `0.0.x` release is a breaking change, so `"0.0.1"` never picks up `0.0.2` by itself. Use the current version when 2c writes the deprecation note.

## Dependencies

- **Blocks:** EPIC-05 (its stories run in `buffoonery`).
- **Built on:** EPIC-01 (the engine) and its children 01a–01e.
- **Related:** the domain-kernel audit (`docs/audit-2026-07-18-domain-kernel.md`).

## Verification

```bash
# new repo (Phase 1)
cargo fmt --check
cargo clippy --all-targets -- -Dclippy::all -Dclippy::pedantic
cargo test
cargo run --example buffoon        # prints 11254

# cardpack (Phase 3)
cargo fmt --check
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo clippy --all-features --all-targets -- -Dclippy::all -Dclippy::pedantic
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo test --all-features
cargo build --no-default-features
cargo deny check bans
git grep -n -i funky -- src Cargo.toml .github Makefile .cargo   # no output
```

Exit criteria:

1. The new repo's suite passes with the counts from Phase 1c, and `buffoon` prints 11254.
2. The new crate is published, and cardpack 0.11.2 names it in its deprecation.
3. Cardpack 0.12.0 passes every gate above with no funky code, feature or build step.
4. `.okf/` validates, and every funky concept points at `buffoonery`.
