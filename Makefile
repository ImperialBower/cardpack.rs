.PHONY: clean build test test-unit test-doc test-wasm build-wasm build_test fmt clippy create_docs ayce default help docs test-nightly clippy-nightly nightly miri mutants tree tree-duplicates deny audit unused-deps install-tools install-nextest install-mutants install-wasm-bindgen-cli watch install-watch

# Default target
default: ayce

# Display help information
help:
	@echo "Available targets:"
	@echo "  make (default)       - Run ayce"
	@echo "  make build           - Build the project"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make test            - Run all tests (nextest for unit, cargo test for doc)"
	@echo "  make test-unit       - Run unit tests via cargo-nextest"
	@echo "  make test-doc        - Run doc tests via cargo test --doc"
	@echo "  make build-wasm      - Build the lib + example for wasm32-unknown-unknown"
	@echo "  make test-wasm       - Run wasm runtime tests (requires wasm-bindgen-cli + node)"
	@echo "  make build_test      - Clean once, then build and test"
	@echo "  make fmt             - Format code"
	@echo "  make clippy          - Run clippy linter"
	@echo "  make create_docs     - Build documentation"
	@echo "  make docs            - Build docs and open in browser"
	@echo "  make ayce            - Run fmt, build_test, clippy, and docs"
	@echo "  make help            - Display this help message"
	@echo ""
	@echo "Nightly:"
	@echo "  make test-nightly    - Run all tests with nightly"
	@echo "  make clippy-nightly  - Run clippy with nightly and deny warnings"
	@echo "  make nightly         - Run nightly test and clippy checks"
	@echo "  make miri            - Run tests under Miri"
	@echo "  make mutants         - Run mutation tests via cargo-mutants"
	@echo "  make unused-deps     - Find unused dependencies with cargo-udeps"
	@echo ""
	@echo "Dependencies and Security:"
	@echo "  make tree            - Show dependency tree"
	@echo "  make tree-duplicates - Show duplicate dependencies"
	@echo "  make deny            - Run full cargo-deny checks"
	@echo "  make audit           - Run advisory-only security audit"
	@echo ""
	@echo "Tools and Workflow:"
	@echo "  make install-tools   - Install cargo-deny, cargo-udeps, cargo-nextest, and cargo-mutants"
	@echo "  make install-nextest - Install cargo-nextest"
	@echo "  make install-mutants - Install cargo-mutants"
	@echo "  make install-wasm-bindgen-cli - Install wasm-bindgen-cli (for test-wasm)"
	@echo "  make watch           - Run cargo-watch for check/test loop"
	@echo "  make install-watch   - Install cargo-watch"
	@echo ""

# Clean build artifacts
clean:
	cargo clean

# Build the project
build:
	cargo build

# Check for cargo-nextest, prompt to install if missing
define check_nextest
	@if ! cargo nextest --version >/dev/null 2>&1; then \
		echo "cargo-nextest is not installed."; \
		printf "Install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install cargo-nextest --locked; \
		else \
			echo "Aborting: cargo-nextest is required for unit tests."; \
			exit 1; \
		fi; \
	fi
endef

# Run unit tests via nextest
test-unit:
	$(check_nextest)
	cargo nextest run

# Run doc tests
test-doc:
	cargo test --doc

# Run all tests: unit tests via nextest, doc tests via cargo test
test: test-unit test-doc

# Build cardpack for wasm32-unknown-unknown across feature combos.
# The repo's .cargo/config.toml supplies the required getrandom backend cfg.
build-wasm:
	@if ! rustup target list --installed | grep -q '^wasm32-unknown-unknown$$'; then \
		echo "Installing wasm32-unknown-unknown target..."; \
		rustup target add wasm32-unknown-unknown; \
	fi
	cargo build --target wasm32-unknown-unknown --all-features
	cargo build --target wasm32-unknown-unknown --no-default-features
	cargo build --target wasm32-unknown-unknown --example wasm

# Check for wasm-bindgen-test-runner, prompt to install if missing.
# wasm-bindgen-test-runner is bundled with wasm-bindgen-cli.
define check_wasm_bindgen_cli
	@if ! command -v wasm-bindgen-test-runner >/dev/null 2>&1; then \
		echo "wasm-bindgen-cli is not installed (provides wasm-bindgen-test-runner)."; \
		printf "Install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install wasm-bindgen-cli; \
		else \
			echo "Aborting: wasm-bindgen-cli is required for wasm runtime tests."; \
			exit 1; \
		fi; \
	fi
	@if ! command -v node >/dev/null 2>&1; then \
		echo "Aborting: node is required for wasm runtime tests."; \
		exit 1; \
	fi
endef

# Run wasm runtime tests under node-headless via wasm-bindgen-test.
test-wasm:
	@if ! rustup target list --installed | grep -q '^wasm32-unknown-unknown$$'; then \
		echo "Installing wasm32-unknown-unknown target..."; \
		rustup target add wasm32-unknown-unknown; \
	fi
	$(check_wasm_bindgen_cli)
	cargo test --target wasm32-unknown-unknown --test wasm

# Check for cargo-mutants, prompt to install if missing
define check_mutants
	@if ! cargo mutants --version >/dev/null 2>&1; then \
		echo "cargo-mutants is not installed."; \
		printf "Install it now? [y/N] "; \
		read answer; \
		if [ "$$answer" = "y" ] || [ "$$answer" = "Y" ]; then \
			cargo install cargo-mutants; \
		else \
			echo "Aborting: cargo-mutants is required for mutation testing."; \
			exit 1; \
		fi; \
	fi
endef

# Run mutation tests
mutants:
	$(check_mutants)
	cargo mutants

# Clean once, then run build + test
build_test: clean build test

# Format code
fmt:
	cargo fmt

# Run clippy linter
clippy:
	cargo clippy -- -W clippy::pedantic

test-nightly:
	cargo +nightly test --all-targets --all-features

clippy-nightly:
	cargo +nightly clippy --lib --all-features -- -D warnings

nightly: test-nightly clippy-nightly

# Run tests under Miri
miri:
	cargo miri test

# Show dependency tree
tree:
	@echo "Showing dependency tree..."
	cargo tree --workspace

# Show duplicate dependencies
tree-duplicates:
	@echo "Showing duplicate dependencies..."
	cargo tree --workspace --duplicates

# Security checks with cargo-deny
deny:
	@echo "Running cargo-deny checks..."
	cargo deny check

# Security audit with cargo-deny (advisories only)
audit:
	@echo "Running security audit..."
	cargo deny check advisories

# Check for unused dependencies (requires nightly)
unused-deps:
	@echo "Checking for unused dependencies..."
	cargo +nightly udeps --workspace --all-features

# Create documentation
create_docs:
	cargo doc --no-deps

# Open documentation in browser
docs: create_docs
	@DOC_PATH="./target/doc/cardpack/index.html"; \
	if command -v xdg-open >/dev/null 2>&1; then \
		xdg-open "$$DOC_PATH"; \
	elif command -v open >/dev/null 2>&1; then \
		open "$$DOC_PATH"; \
	else \
		echo "No supported opener found (tried xdg-open and open)."; \
		echo "Open $$DOC_PATH manually."; \
		exit 1; \
	fi

# All You Can Eat - Run all checks
ayce: fmt build_test clippy create_docs

# Install cargo-nextest
install-nextest:
	cargo install cargo-nextest --locked

# Install cargo-mutants
install-mutants:
	cargo install cargo-mutants

# Install wasm-bindgen-cli (provides wasm-bindgen-test-runner used by `make test-wasm`)
install-wasm-bindgen-cli:
	cargo install wasm-bindgen-cli

# Install required tools
install-tools:
	@echo "Installing development tools..."
	cargo install cargo-deny
	cargo install cargo-udeps
	cargo install cargo-nextest --locked
	cargo install cargo-mutants
	@echo ""
	@echo "Tools installed!"
	@echo ""

# Watch mode for development (requires cargo-watch)
watch:
	cargo watch -x "check --workspace" -x "test --workspace"

# Install cargo-watch
install-watch:
	cargo install cargo-watch
