# Update Log

## 2026-07-23
* **Examples flag-free**: added a self dev-dependency (`cardpack = { path = ".", features = ["full", "funky"] }`, gated off wasm/bare-metal) so `cargo run --example <x>` needs no `--features`; `default = []` is untouched for consumers. Removed the now-non-pure host `test --no-default-features --lib` and `build --no-default-features --examples` CI steps (example purity now gated by the target jobs); README example commands are bare again. New [decision](decisions/examples-self-dev-dependency.md); [feature flags](architecture/feature-flags.md) gotcha added.
* **Feature graph**: `funky` now implies `serde` (`funky = ["std", "serde"]`) — the `funky/types/*` files use serde unconditionally, so `--features funky` alone left `serde` unlinked and the `buffoon`/`funky_tour` examples failed to compile ([feature flags](architecture/feature-flags.md), [funky engine](architecture/funky-engine.md)).
* **Decks**: Added Mughal (96) and Dashavatara (120) Ganjifa decks — [deck catalog](decks/deck-catalog.md) now covers 14 kinds; [localization](architecture/localization.md) gains the `mughal`/`dashavatara` fluent bases (EPIC-02).
* **Decision**: DeckKind is now #[non_exhaustive] and the crate bumped to 0.9.0 — adding enum variants was semver-major vs 0.8.1; future deck additions become non-breaking ([deck catalog](decks/deck-catalog.md)).

## 2026-07-22
* **Creation**: Authored the initial knowledge set — 12 concepts across [architecture](architecture/) (crate overview, card model, feature flags, domain kernel, funky engine, localization), [decks](decks/) (catalog, extension playbook), [workflows](workflows/) (build/test, wasm), [decisions](decisions/) (std_rng unconditional, std-io outside full), and [references](references/) (docs map), with per-directory indexes.
* **Creation**: Scaffolded the cardpack.rs Knowledge Bundle bundle with `okf_init.py` — see [getting started](getting-started.md).
