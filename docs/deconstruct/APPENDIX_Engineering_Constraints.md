# Appendix: Engineering Constraints

> **NON-NORMATIVE.** Nothing in this document binds a rebuild. It records
> engineering context about the *original* — runtime environments, optional
> capability layering, toolchain pinning, CI enforcement, and performance
> posture — for a rebuilder's situational awareness only. No DECON epic
> depends on any claim made here; `path:line` citations below are
> provenance, exactly as in every epic's own Provenance section, except
> that this entire document carries that status, not just one section of
> it.

## Runtime spectrum

The original runs across a real spectrum of hosting environments, each
built and tested in CI, not just claimed:

- **Full OS** — the default build, with every optional capability
  available.
- **Bare-metal, allocator present, no operating system** — a
  `no_std` + `alloc` build (heap allocation required; a fully
  allocation-free mode does not exist and was an explicit non-goal:
  `docs/superpowers/specs/2026-05-01-no-std-alloc-design.md` §2). Verified
  by a compile-and-link-only smoke binary cross-built for
  `thumbv7em-none-eabihf` (`examples/no_std_smoke.rs`); the smoke binary
  is never executed under emulation — `cargo build`'s exit code is the
  verification bar (design doc §7.1).
- **Browser / WebAssembly** — `wasm32-unknown-unknown`, exercised at
  runtime (not just compiled) by `tests/wasm.rs` under `wasm-bindgen-test`,
  covering seeded-shuffle determinism, draw, no-arg shuffle entropy
  sourcing, deck-registry enumeration, and in-memory data parsing.

## Optional capability layering

Four feature flags, all default-on, each strictly additive over a minimal
`alloc`-only core:

| Capability | Gates | Requires |
|---|---|---|
| Internationalization (i18n) | Localized names, resolution machinery | `std` |
| Colored terminal display | ANSI-colored card/pile rendering | `std` |
| YAML | File/string-based card-list round-trips, one deck's data-driven load path | `std` (implies `serde`) |
| Structured serialization | Round-trip through a structured encoding | `alloc` only |

`Cargo.toml`'s documented guarantee: a default-features build behaves the
same release to release, aside from explicitly flagged breaking changes
(`README.md:98`; `CHANGELOG.md` Breaking sections). A fifth flag, `std`,
gates only the process-default-RNG convenience shuffle/draw path — every
other shuffle mode (seeded, caller-supplied RNG) works without it.

## Toolchain

MSRV pinned at **1.85**, edition 2024 (`Cargo.toml:8-9`); no MSRV bump
without explicit discussion (`CONTRIBUTING.md:15-16`). CI's build matrix
runs `beta`, `stable`, and the pinned `1.85.0` (`.github/workflows/CI.yaml:20`).

## CI platform matrix

`.github/workflows/CI.yaml` enforces the runtime spectrum above as gated
jobs, not just documentation:

- `no-std-build` (`:128`) — `--no-default-features` build and
  `--lib`-scoped test run, plus a `serde`-only combination.
- `no-std-thumbv7em` (`:147-159`) — cross-builds `--no-default-features`
  for the bare-metal `thumbv7em-none-eabihf` target, with and without
  `serde`, including the no_std smoke example — a genuine bare-metal link
  target in CI, not merely aspirational.
- `wasm-build` (`:80-92`) — builds for `wasm32-unknown-unknown` under
  `--all-features` and `--no-default-features`, plus the wasm example.
- `wasm-test` (`:94-107`) — runs `tests/wasm.rs` under a real wasm runtime
  (`wasm-bindgen-cli`), the one job in the matrix that executes wasm code
  rather than only compiling it.

## Performance posture

- `benches/draw.rs` (Criterion, `harness = false`) benchmarks draw,
  shuffle (both plain and seeded), pile concatenation, and 5-of-7
  combination enumeration against the largest shipped deck (108 cards) —
  regression-tracked release-to-release by relative delta, not an
  absolute-number contract (`benches/draw.rs:1-17`).
- Vocabulary construction is compile-time work in the original: facet and
  card constructors are `const fn` (`src/basic/types/pips.rs:118`,
  `src/basic/types/basic_card.rs:58`), so a shipped deck's static card
  table costs nothing at runtime beyond referencing already-built data.
- This performance characteristic is **informative only** — SD-03 (indexed
  in `MANIFEST.md`) records that no epic in this pack binds a rebuild to
  matching the original's benchmark numbers or construction-time strategy.

## Seeded-shuffle stability disclaimer

Seeded-shuffle determinism (DECON-03) is guaranteed by the original only
**within one major version of its own randomness-generation dependency** —
a version bump to that dependency can silently change every seeded
shuffle's output, and the original's own design documentation states this
as an accepted tradeoff, not a defect
(`docs/2026-04-29-seeded-shuffle-design.md` §3). This is why DECON-03's
seeded-shuffle vector is marked informative (SD-01) rather than a
byte-exact cross-implementation contract: the original itself does not
promise cross-version, let alone cross-language, exact reproducibility for
that capability. A caller-supplied-RNG escape hatch exists precisely so a
consumer can opt into their own stability guarantee instead of relying on
the original's default.
