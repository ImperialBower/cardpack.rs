# Architecture

* [cardpack — Generic Deck of Cards](crate-overview.md) - Pure-by-default no_std Rust library modeling playing-card decks, with opt-in std, i18n, serde, YAML, and a Balatro-style engine.
* [Card model — Pip, BasicCard, Card, Pile](card-model.md) - The four-layer generic card model and the traits (DeckedBase, Decked, Ranged) that turn a type into a deck.
* [Cargo feature flags](feature-flags.md) - The pure-by-default feature matrix — default is an alloc-only no_std kernel; std, i18n, color, yaml, serde, std-io, and funky are opt-in.
* [Domain-kernel posture](domain-kernel.md) - cardpack is a value-type domain kernel — pure by default, no I/O in the core, format crates kept out of the public API.
* [Funky — the Balatro-style engine](funky-engine.md) - std-only feature-gated module modeling Balatro cards, jokers, scoring, shop, and vouchers on top of the core card model.
* [Localization (Fluent i18n)](localization.md) - Card, rank, and suit names resolve per locale via Project Fluent — en-US, de, fr, la, tlh — behind the i18n feature.
