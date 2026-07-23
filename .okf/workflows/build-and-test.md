---
type: Playbook
title: Build, test, and quality gates
description: make (ayce) is the everything gate — fmt, build, three test layers, clippy-pedantic, MSRV, no_std, docs; plus mutants, miri, coverage, and deny.
tags: [workflow, make, ci, testing]
timestamp: 2026-07-22T13:10:00Z
---

# Day-to-day

```shell
make            # default target `ayce` ("all you can eat"):
                # fmt → build → test-unit/doc/std-io → clippy → msrv → no-std → docs
make help       # list all targets
```

Test layers: `test-unit`, `test-doc` (101+ doctests), `test-std-io` (the
filesystem seam gets its own pass since `std-io` is outside `full` —
[decision](/decisions/std-io-outside-full.md)).

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

# Conventions

* Clippy runs at `-Dpedantic --all-targets` and must stay clean; `unwrap`/
  `expect` are only allowed under `cfg(test)`.
* CI is `.github/workflows/CI.yaml` (build/test badge on the README); codecov
  tracks coverage.
* Test framework: `rstest` for fixture/case tables.

# Citations

[1] [Makefile](../../Makefile)
[2] [.github/workflows/CI.yaml](../../.github/workflows/CI.yaml)
