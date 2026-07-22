---
type: Playbook
title: WebAssembly support
description: cardpack compiles to wasm32-unknown-unknown with every feature combination; consumers must configure the getrandom wasm_js backend.
tags: [wasm, portability, getrandom]
timestamp: 2026-07-22T13:10:00Z
---

# Status

The crate builds cleanly for `wasm32-unknown-unknown` (browser WASM) with
every feature combination, a direct payoff of the
[domain-kernel](/architecture/domain-kernel.md) posture.

# The getrandom seam

`rand 0.9 → rand_core 0.9 → getrandom 0.3` forces wasm32 consumers to pick a
backend, else builds fail with "no backend selected". cardpack pre-enables the
`wasm_js` feature on `getrandom` for wasm targets in its own Cargo.toml, but
**consumers still must set the cfg flag themselves**, typically in
`.cargo/config.toml`:

```toml
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
```

# Where to look

* [docs/wasm.md](../../docs/wasm.md) — backend setup, recommended feature
  combos, runtime gotchas.
* [examples/wasm.rs](../../examples/wasm.rs) — working example.
* `tests/wasm.rs` — wasm-bindgen-test suite (`make test-wasm`).

Seeded shuffle works on wasm and no_std alike because `StdRng` comes from
rand's `std_rng` feature, enabled unconditionally —
[see the decision](/decisions/rand-std-rng-unconditional.md).

# Citations

[1] [docs/wasm.md](../../docs/wasm.md)
[2] [Cargo.toml wasm32 target section](../../Cargo.toml)
