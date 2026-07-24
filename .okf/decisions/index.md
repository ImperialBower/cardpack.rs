# Decisions

* [rand's std_rng stays unconditional](rand-std-rng-unconditional.md) - rand's std_rng feature is deliberately NOT gated on cardpack's std feature — moving it re-breaks seeded shuffle for no_std consumers.
* [std-io is excluded from full](std-io-outside-full.md) - The one filesystem API (cards_from_yaml_file) sits behind its own std-io feature, deliberately left out of the full umbrella so both kernel and convenience stack stay I/O-free.
* [Examples are flag-free via a self dev-dependency](examples-self-dev-dependency.md) - A self dev-dependency force-enables full+funky for host dev-artifact builds so `cargo run --example X` needs no flags; keeps default=[] for consumers but moves example-purity gating to the target (bare-metal/wasm) jobs.
