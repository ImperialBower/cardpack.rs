# Decisions

* [rand's std_rng stays unconditional](rand-std-rng-unconditional.md) - rand's std_rng feature is deliberately NOT gated on cardpack's std feature — moving it re-breaks seeded shuffle for no_std consumers.
* [std-io is excluded from full](std-io-outside-full.md) - The one filesystem API (cards_from_yaml_file) sits behind its own std-io feature, deliberately left out of the full umbrella so both kernel and convenience stack stay I/O-free.
