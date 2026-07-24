---
type: Decision
title: Examples are flag-free via a cargo alias, never a self dev-dependency
description: "`cargo ex <name>` aliases `run --features full,funky --example`, keeping example ergonomics in developer tooling; a self dev-dependency was tried and reverted because it silently breaks `cargo deny check bans` and every host purity gate."
tags: [decision, purity, features, examples, ci, cargo-deny]
timestamp: 2026-07-23T00:00:00Z
---

# Decision

Examples are made flag-free by an **alias in `.cargo/config.toml`**:

```toml
[alias]
ex = "run --features full,funky --example"
```

So `cargo ex demo` replaces `cargo run --features full,funky --example demo`.

`Cargo.toml` must **not** carry a self dev-dependency
(`cardpack = { path = ".", features = ["full", "funky"] }`) for this purpose.
That approach was implemented, then reverted — see below.

# Why

The crate is pure by default (`default = []`, Invariant 3 of the
[domain kernel](/architecture/domain-kernel.md)), so every example touching
`std`/`i18n`/`colored-display`/`yaml`/`funky` needs a `--features` flag.
The friction is real, but it belongs in **developer tooling**, not in the
dependency graph. An alias is invisible to Cargo's resolver, so every purity
gate keeps working and consumers are unaffected.

# Why the self dev-dependency fails (load-bearing)

A self dev-dep does **not** merely add a `(dev)` edge — it force-enables
`full`+`funky` on the `cardpack` node of `cargo metadata`'s **feature-unified
resolve**. Tools that read that resolve therefore see `colored`,
`serde_norway`, and `fluent-templates` as **normal** dependencies of the
kernel:

```
error[banned]: crate 'colored = 3.1.1' is explicitly banned
   ├ colored v3.1.1
     └── cardpack v0.9.0        # <- no (dev) marker
```

`cargo deny check bans` — the second step of the `kernel-purity` CI job —
fails, and **no cargo-deny setting can suppress it**. All of these were tried
and all still report the four banned crates:

* `cargo deny --exclude-dev check bans`
* `[graph] exclude-dev = true`
* `[graph] no-default-features = true`
* the two combined

The dev *edge* is removable; the feature *activation* is not.

Two further casualties:

* `wildcards = "deny"` flags `cardpack = { path = "." }` itself — a path
  dependency carries no version requirement, so it reads as a wildcard.
* The host purity gates stop testing anything, because Cargo builds all dev
  artifacts with the features on: `cargo test --no-default-features --lib` is
  no longer a pure lib test, and `cargo build --no-default-features --examples`
  no longer catches a missing `required-features` declaration (nothing is
  skipped anymore).

Note that `cargo tree --locked --no-default-features --edges normal` stays
**clean** throughout — `cargo tree` resolves features per-target correctly.
That divergence is the trap: the first step of `kernel-purity` passes while
the second fails, in the same job, for the same tree.

# How to apply

* Add new examples to the `[[example]]` list with accurate
  `required-features`; they are what `cargo build --no-default-features
  --examples` enforces.
* Document the run command in the example's `# Features` doc header as
  `cargo ex <name>`.
* If an example needs a feature outside `full`+`funky` (e.g. `std-io`), extend
  the alias or run it explicitly — do not add the feature to `default`.
* Do **not** reintroduce a self dev-dependency to "improve" ergonomics, and do
  not silence the resulting cargo-deny failure with `[bans].skip` or by
  deleting entries from `[bans].deny` — that removes the only automated proof
  of [kernel purity](/architecture/domain-kernel.md).
* Do **not** solve example friction by putting features in `default` instead —
  that violates Invariant 3 and breaks every no_std/wasm/bare-metal consumer
  ([feature flags](/architecture/feature-flags.md)).

# Citations

[1] [.cargo/config.toml alias](../../.cargo/config.toml)
[2] [Cargo.toml dev-dependencies warning comment](../../Cargo.toml)
[3] [.github/workflows/CI.yaml `no-std-build` and `kernel-purity` jobs](../../.github/workflows/CI.yaml)
[4] [deny.toml `[bans].deny` list](../../deny.toml)
[5] [domain kernel — Invariant 3](/architecture/domain-kernel.md)
