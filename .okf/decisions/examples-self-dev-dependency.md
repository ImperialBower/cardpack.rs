---
type: Decision
title: Examples are flag-free via a self dev-dependency
description: A self dev-dependency force-enables full+funky for host dev-artifact builds so `cargo run --example X` needs no flags — keeping default=[] for consumers, at the cost of host no-default example/lib-test purity gates.
tags: [decision, purity, features, examples, ci]
timestamp: 2026-07-23T00:00:00Z
---

# Decision

`Cargo.toml` carries a **self dev-dependency**:

```toml
[target.'cfg(all(not(target_arch = "wasm32"), not(target_os = "none")))'.dev-dependencies]
cardpack = { path = ".", features = ["full", "funky"] }
```

Its only job is to make every example runnable with a bare
`cargo run --example <x>` — no `--features` needed. Cargo compiles
dev-dependencies **only** when building dev artifacts (examples/tests/benches),
so this force-enables `full`+`funky` for those builds and nothing else.

# Why

The crate is pure by default (`default = []`, Invariant 3 of the
[domain kernel](/architecture/domain-kernel.md)), so every example that touches
`std`/`i18n`/`colored-display`/`yaml`/`funky` otherwise errors with
"requires the features …". Passing the right flag per example is friction. A
self dev-dependency removes it **without** changing `default`, so downstream
consumers still get the pure kernel — verified by the plain
`cargo build --no-default-features` and `kernel-purity` (cargo-tree) CI gates,
which compile no dev-dependencies and are untouched by this.

# The trade-off (load-bearing — read before "cleaning up")

Because the self dev-dep force-enables features for **all host dev-artifact
builds**, two purity checks can no longer run on the host:

* `cargo test --no-default-features --lib` — the lib now compiles with
  `full`+`funky`, so it is no longer a *pure* lib test.
* `cargo build --no-default-features --examples` — every example now builds
  (nothing is skipped), so it no longer catches a missing `required-features`
  declaration.

Both were removed from the `no-std-build` CI job. Purity is still gated
authoritatively by paths the target-gated self dev-dep **cannot reach**:

* the two plain `cargo build --no-default-features[  --features serde]` builds
  (no dev-deps compiled),
* `no-std-thumbv7em` — builds `no_std_smoke` against the pure lib on bare metal,
* the wasm job — builds the `wasm` example against the pure lib,
* `kernel-purity` — cargo-tree over the pure tree.

# How to apply

* Keep `required-features` on the examples anyway — they document intent and
  re-arm the protection if the self dev-dep is ever removed.
* Do **not** "fix purity" by adding a host `--no-default-features --examples`
  step back — it will silently build with all features and mislead. Purity
  belongs to the target jobs above.
* Do **not** solve example friction by putting features in `default` instead —
  that violates Invariant 3 and breaks every no_std/wasm/bare-metal consumer
  ([feature flags](/architecture/feature-flags.md)).
* The self dev-dep is gated off `wasm32` and `target_os = "none"` because
  `full`/`funky` need `std`; keep it that way or the wasm/bare-metal builds
  break.

# Citations

[1] [Cargo.toml self dev-dependency comment](../../Cargo.toml)
[2] [.github/workflows/CI.yaml `no-std-build` job](../../.github/workflows/CI.yaml)
[3] [domain kernel — Invariant 3](/architecture/domain-kernel.md)
