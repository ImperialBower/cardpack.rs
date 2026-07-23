# Workflows

* [Build, test, and quality gates](build-and-test.md) - make (ayce) is the everything gate — fmt, build, three test layers, clippy-pedantic, MSRV, no_std, docs; plus mutants, miri, coverage, and deny.
* [WebAssembly support](wasm.md) - cardpack compiles to wasm32-unknown-unknown with every feature combination; consumers must configure the getrandom wasm_js backend.
