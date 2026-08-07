---
type: Playbook
title: WebAssembly support
description: cardpack compiles to wasm32-unknown-unknown with every feature combination; consumers must configure the getrandom wasm_js backend.
tags: [wasm, portability, getrandom]
timestamp: 2026-08-06T00:00:00Z
---

# Status

The crate builds cleanly for `wasm32-unknown-unknown` (browser WASM) with
every feature combination, a direct payoff of the
[domain-kernel](/architecture/domain-kernel.md) posture.

# The getrandom seam

`rand`'s wasm32 entropy source depends on `getrandom`, which forces wasm32
consumers to pick a backend, else builds fail with "no backend selected".
cardpack pre-enables the `wasm_js` feature on `getrandom` for wasm targets in
its own Cargo.toml, but **consumers still must set the cfg flag themselves**,
typically in `.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```

**Two `getrandom` majors must be pinned, not one.** `rand 0.10` depends
directly on `getrandom 0.4` for its wasm32 backend, while `getrandom 0.3`
still reaches the graph transitively (via `rand_core 0.9`, pulled in by
dev-deps). The rand 0.9→0.10 dependabot bump only had a 0.3 pin, so CI's
`wasm-build`/`wasm-test` jobs broke with `getrandom 0.4`'s "not supported by
default; you may need to enable the wasm_js crate feature" compile error —
the crate never calls `getrandom` directly, so `cargo update` alone doesn't
surface the second pin as missing. Fix is a second Cargo.toml entry under
`[target.'cfg(target_arch = "wasm32")'.dependencies]`, renamed via
`package = "getrandom"` since the section can't repeat a bare `getrandom`
key twice:

```toml
getrandom = { version = "0.3", features = ["wasm_js"] }
getrandom_v4 = { package = "getrandom", version = "0.4", features = ["wasm_js"] }
```

Cargo unifies features onto the one resolved package per version regardless
of the manifest key it's declared under, so `getrandom_v4` never needs to be
referenced from source — its only job is turning the feature on.

# Where to look

* [wasm guide](/references/wasm-guide.md) (`docs/wasm.md`) — backend setup,
  recommended feature combos, runtime gotchas.
* [examples/wasm.rs](../../examples/wasm.rs) — working example.
* `tests/wasm.rs` — wasm-bindgen-test suite (`make test-wasm`).

Seeded shuffle works on wasm and no_std alike because `StdRng` comes from
rand's `std_rng` feature, enabled unconditionally —
[see the decision](/decisions/rand-std-rng-unconditional.md).

# Citations

[1] [docs/wasm.md](/references/wasm-guide.md)
[2] [Cargo.toml wasm32 target section](../../Cargo.toml)
