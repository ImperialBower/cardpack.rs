# Contributing to cardpack

Thanks for considering a contribution. This doc covers the project
conventions so review goes smoothly.

## Getting started

```sh
git clone https://github.com/ImperialBower/cardpack.rs.git
cd cardpack.rs
make install-tools   # cargo-deny, cargo-udeps, cargo-nextest, cargo-mutants
make ayce            # the umbrella check (fmt + build + test + clippy + docs)
```

MSRV is **1.85** (edition 2024). Don't use features that bumped the MSRV
beyond that without an explicit discussion in the PR.

## Code style

- `cargo fmt --all` is enforced; `make fmt` formats in place.
- Clippy lints are strict: `cargo clippy -- -Dclippy::all
  -Dclippy::pedantic`. A few lints are project-allowed in `src/lib.rs`
  (see the `#![allow(...)]` block at the top); leave those alone unless
  there's a reason.
- Test functions use a `topic__sub_case` snake_case convention with
  double-underscore separators (e.g. `shuffled_with_seed__is_deterministic`).
- Default to writing no comments. Prefer well-named functions and
  variables. Add a comment only when the *why* is non-obvious.

## Running tests

| Command | What it does |
|---|---|
| `make test` | Unit tests (nextest) + doctests |
| `make test-unit` | Unit tests only |
| `make test-doc` | Doctests only |
| `cargo test --test properties` | Property tests via proptest |
| `make build-wasm` | Build the lib + example for wasm32-unknown-unknown |
| `make test-wasm` | Wasm runtime tests (requires wasm-bindgen-cli + node) |
| `make ayce` | Umbrella: fmt + build + test + clippy + docs |
| `make mutants` | Mutation testing (slow; run before major changes) |

## Feature flags

cardpack ships four Cargo features (all in `default`):

- `i18n` — `fluent-templates`-backed localization
- `colored-display` — `colored` crate ANSI output and the `colors()`
  trait method
- `yaml` — `serde_norway`-backed YAML deck loading + the `Razz` deck
- `serde` — `Serialize` / `Deserialize` derives on `Pip`, `Card`,
  `Pile`, etc.

Any change that introduces or modifies feature-gated code should be
verified across the matrix:

```sh
cargo build
cargo build --no-default-features
cargo build --no-default-features --features i18n
cargo build --no-default-features --features colored-display
cargo build --no-default-features --features yaml
cargo build --no-default-features --features serde
cargo build --all-features
cargo test
cargo test --no-default-features
cargo test --all-features
```

CI runs the matrix on every push (see `.github/workflows/CI.yaml`).

When you add a `#[cfg(feature = "X")]` to an enum variant, every
`match` arm in the codebase that references that variant needs the
same `#[cfg]` attribute — see `src/basic/decks/registry.rs` for the
pattern with `Razz`.

## WebAssembly

cardpack compiles to `wasm32-unknown-unknown` across all feature
combinations. The CI jobs `wasm-build` and `wasm-test` guard this. If
you're adding code that uses `std::fs`, threading, time, or other
panic-on-wasm APIs, gate it behind a feature or document the runtime
caveat in `docs/wasm.md`.

## Adding a new locale

1. Create `src/localization/locales/<bcp47>/{french,skat,tarot}.ftl`
   following the en-US schema.
2. Add a `README.md` in the locale directory documenting per-file
   confidence levels and the reviewer profile required to promote each
   file from DRAFT to REVIEWED.
3. Add a `<lang>_locale_is_wired` test in `src/localization.rs` that
   asserts a high-confidence entry resolves correctly.
4. Optionally add a constant on the `Named` trait
   (`pub const FRANCAIS: LanguageIdentifier = ...`) so the locale
   shows up in `examples/demo.rs --verbose` and downstream code can
   reference it ergonomically.
5. If the locale uses a non-English connector for compound names
   (e.g. French "Roi *de* Cœur", Latin's bare-juxtaposition genitive),
   update `Card::fluent_connector` in `src/basic/types/card.rs`.

See the existing `fr/`, `la/`, `tlh/` drafts for working examples.

## Audit doc as roadmap

Open work is tracked in [`docs/audit-2026-04-29.md`](docs/audit-2026-04-29.md)
(and predecessor `docs/audit-2026-04-09.md`). The §16 prioritized
action table is the closest thing to a roadmap. If your PR closes an
audit row, update the row's status (✅ done / 🟡 partial) and link the
PR.

## Pull requests

- Run `make ayce` locally before pushing. CI will catch what `ayce`
  doesn't (wasm jobs, semver-checks), but `ayce` should be green.
- Update `CHANGELOG.md` in the `[Unreleased]` section.
- Link to the audit row if your PR closes one.
- Keep commits focused. Bundling unrelated refactors with a feature
  change makes review harder.

## Reporting bugs / proposing features

Open a [GitHub issue](https://github.com/ImperialBower/cardpack.rs/issues)
with a minimal reproducer (for bugs) or a use case + proposed API
sketch (for features). For larger changes, a design doc in `docs/`
following the pattern of `docs/2026-04-29-seeded-shuffle-design.md` is
welcome.
