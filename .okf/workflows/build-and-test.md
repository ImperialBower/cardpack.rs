---
type: Playbook
title: Build, test, and quality gates
description: make (ayce) is the everything gate — fmt, build, four test layers, clippy-pedantic, MSRV, no_std, docs; plus mutants, miri, coverage, and deny.
tags: [workflow, make, ci, testing]
timestamp: 2026-08-25T12:00:00Z
---

# Day-to-day

```shell
make            # default target `ayce` ("all you can eat"):
                # fmt → build → test-unit/doc/std-io/funky/crypto → clippy
                # → msrv → no-std → docs
make help       # list all targets
```

Test layers: `test-unit`, `test-doc` (101+ doctests), `test-std-io` (the
filesystem seam gets its own pass since `std-io` is outside `full` —
[decision](/decisions/std-io-outside-full.md)), `test-funky` (the `funky`
feature is `std`-only and off by default, so it's outside `full` too;
previously only `msrv`'s pinned 1.85.0 toolchain compiled it, which meant a
missing MSRV toolchain silently skipped the entire feature on a stable-only
machine — this is how the rand 0.10 `RngExt` split shipped broken to `main`
via a dependabot merge. `test-funky` runs it on stable so it can't hide), and `test-crypto`
(`--features full,crypto,seal-test-double` — the sealed-deck backends and the
seal test double are all outside `full`, [decision](/decisions/crypto-features-outside-full.md)).

**Why `seal-test-double` rides along with `test-crypto`.** Its module is
`cfg(any(test, feature = "seal-test-double"))`, so its *unit* tests compile
under a bare `cargo test`. Its **doctests do not**: a doctest is built as an
outside consumer of the crate, where `cfg(test)` is false, so the example on
`seal_roundtrip` only exists when the feature is on. Without the feature in
`test-crypto` that doctest was dark — the `funky` lesson again, in a form the
`funky` fix would not have caught.

# Portability gates

* `make no-std` — builds `--no-default-features` including bare-metal
  (`thumbv7em-none-eabihf`); dev-deps are target-gated in Cargo.toml so this
  works.
* `make build-wasm` / `make test-wasm` — wasm32 ([wasm](/workflows/wasm.md)).
* `make msrv` — Rust 1.85 check.

# Deeper verification

* `make mutants` — cargo-mutants mutation testing.
* `make miri`, `make coverage` (llvm-cov → codecov), `make bench` (criterion
  `benches/draw.rs`), `make deny` / `make audit` / `make unused-deps`.
* `make nightly` — nightly test + clippy.
* Property tests: `tests/properties.rs` (proptest).

# Golden YAML fixtures

`make yaml-fixtures` runs `cargo ex yaml_decks`, regenerating one fixture per
`DeckKind` into `tests/fixtures/yaml/`. It is **idempotent**: run it twice and
`git status` is clean the second time, so CI can assert
`git diff --exit-code tests/fixtures/yaml/`.

`tests/yaml_golden.rs` compares generated output to those files **byte-for-byte**,
which makes `serde_norway`'s formatting part of the contract — a dependency bump
that changes quoting or indentation is *supposed* to turn it red. Regenerate,
review the diff, note it in the CHANGELOG; do not loosen the comparison
([envelope decision](/decisions/yaml-envelope-format.md)). A companion test
pins the fixture count to `DeckKind::all().len()`, so a 15th deck cannot land
without a fixture.

# Conventions

* Clippy runs at `-Dpedantic --all-targets` and must stay clean; `unwrap`/
  `expect` are only allowed under `cfg(test)`.
* CI is `.github/workflows/CI.yaml` (build/test badge on the README); codecov
  tracks coverage. The `clippy` job (the one job that compiles `--all-features
  --all-targets`, covering `funky`) runs on `pull_request` events too — it
  used to be push-only, which is how a dependabot PR merged with a
  `--all-features` build break unreviewed. `fmt`/`doc` are still push-only.
* Test framework: `rstest` for fixture/case tables.

# Citations

[1] [Makefile](../../Makefile)
[2] [.github/workflows/CI.yaml](../../.github/workflows/CI.yaml)
