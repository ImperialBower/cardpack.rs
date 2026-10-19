---
type: Playbook
title: WebAssembly support
description: cardpack compiles to wasm32-unknown-unknown with every feature combination; consumers must configure the getrandom wasm_js backend.
tags: [wasm, portability, getrandom]
timestamp: 2026-10-18T00:00:00Z
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

**Pin the `getrandom` major that `rand` uses, and only that one.**
`rand 0.10` depends directly on `getrandom 0.4` for its wasm32 backend. The
rand 0.9→0.10 dependabot bump kept only the old 0.3 pin, so CI's
`wasm-build`/`wasm-test` jobs broke with `getrandom 0.4`'s "not supported by
default; you may need to enable the wasm_js crate feature" compile error —
the crate never calls `getrandom` directly, so `cargo update` alone doesn't
surface the missing pin. The fix added a 0.4 entry under
`[target.'cfg(target_arch = "wasm32")'.dependencies]`, renamed via
`package = "getrandom"`:

```toml
getrandom_v4 = { package = "getrandom", version = "0.4", features = ["wasm_js"] }
```

The 0.3 pin stayed beside it until 2026-10-18, on the belief that dev-deps
still needed it. They do not on wasm32: the only 0.3 user is `proptest` (via
`rand_core 0.9`), which is gated off wasm32. The leftover pin gave every
consumer a duplicate `getrandom`, so `buffoonery` had to skip it in
`cargo deny`. It was removed (EPIC-06 2f); `cargo tree -i getrandom@0.3.4
--target wasm32-unknown-unknown` must stay empty. Before adding a pin back,
run that command and see who needs it.

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
