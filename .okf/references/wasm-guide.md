---
type: Reference
title: Using cardpack on WebAssembly
description: "Consumer-side setup for wasm32-unknown-unknown: the getrandom wasm_js backend, the verified feature matrix, and runtime gotchas."
tags: [wasm, workflow, consumer-setup]
resource: https://github.com/ImperialBower/cardpack.rs/blob/main/docs/wasm.md
timestamp: 2026-07-26T12:00:00Z
---

# What it covers

The consumer-side guide to running cardpack in the browser: the
`.cargo/config.toml` entries a consuming crate needs, which feature
combinations are verified against `wasm32-unknown-unknown`, and the runtime
gotchas that only show up in a browser.

# Authoritative for

* **The `getrandom` backend requirement.** cardpack compiles to wasm cleanly,
  but a consumer must select the `wasm_js` backend themselves — this is the
  detail that makes seeded shuffle work in a browser. The bundle's distilled
  version is the [wasm workflow](/workflows/wasm.md).

# In-repo path

`docs/wasm.md`

This concept is a pointer, not a copy — the linked document is authoritative.
