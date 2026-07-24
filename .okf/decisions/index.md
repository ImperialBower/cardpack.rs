# Decisions

* [rand's std_rng stays unconditional](rand-std-rng-unconditional.md) - rand's std_rng feature is deliberately NOT gated on cardpack's std feature — moving it re-breaks seeded shuffle for no_std consumers.
* [std-io is excluded from full](std-io-outside-full.md) - The one filesystem API (cards_from_yaml_file) sits behind its own std-io feature, deliberately left out of the full umbrella so both kernel and convenience stack stay I/O-free.
* [Examples are flag-free via a cargo alias, never a self dev-dependency](examples-flag-free-alias.md) - `cargo ex <name>` supplies the features from .cargo/config.toml; a self dev-dependency was tried and reverted because it feature-activates full+funky on the crate's own metadata node, which no cargo-deny setting can undo.
