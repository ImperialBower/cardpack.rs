.PHONY: clean build test build_test fmt clippy create_docs docs ayce default help \
        test-nightly clippy-nightly nightly \
        tree tree-duplicates deny audit unused-deps \
        install-tools watch install-watch

# Default target
default: ayce

# Display help information
help:
	@echo "Available targets:"
	@echo "  make (default)       - Run ayce"
	@echo "  make build           - Build the project"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make test            - Run tests"
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
	@echo "  make unused-deps     - Find unused dependencies with cargo-udeps"
	@echo ""
	@echo "Dependencies and Security:"
	@echo "  make tree            - Show dependency tree"
	@echo "  make tree-duplicates - Show duplicate dependencies"
	@echo "  make deny            - Run full cargo-deny checks"
	@echo "  make audit           - Run advisory-only security audit"
	@echo ""
	@echo "Tools and Workflow:"
	@echo "  make install-tools   - Install cargo-deny and cargo-udeps"
	@echo "  make watch           - Run cargo-watch for check/test loop"
	@echo "  make install-watch   - Install cargo-watch"
	@echo ""

# Clean build artifacts
clean:
	cargo clean

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Clean once, then run build + test
build_test: clean build test

# Format code
fmt:
	cargo fmt

# Run clippy linter (lint levels are configured in src/lib.rs)
clippy:
	cargo clippy

# Run all tests with nightly
test-nightly:
	cargo +nightly test --all-targets --all-features

# Run clippy with nightly and deny warnings
clippy-nightly:
	cargo +nightly clippy --lib --all-features -- -D warnings

# Run nightly test and clippy checks
nightly: test-nightly clippy-nightly

# Show dependency tree
tree:
	@echo "Showing dependency tree..."
	cargo tree

# Show duplicate dependencies
tree-duplicates:
	@echo "Showing duplicate dependencies..."
	cargo tree --duplicates

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
	cargo +nightly udeps --all-features

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

# Install required tools
install-tools:
	@echo "Installing development tools..."
	cargo install cargo-deny
	cargo install cargo-udeps
	@echo ""
	@echo "Tools installed!"
	@echo ""

# Watch mode for development (requires cargo-watch)
watch:
	cargo watch -x "check" -x "test"

# Install cargo-watch
install-watch:
	cargo install cargo-watch
