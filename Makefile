.PHONY: help build run test check fmt lint clean release dev

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build the project
	cargo build

run: ## Run the application
	cargo run

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with output
	cargo test -- --nocapture

check: ## Quick compile check
	cargo check

fmt: ## Format code
	cargo fmt

fmt-check: ## Check code formatting
	cargo fmt -- --check

lint: ## Run clippy linter
	cargo clippy -- -D warnings

clean: ## Clean build artifacts
	cargo clean

release: ## Build optimized release version
	cargo build --release

run-release: ## Run release version
	cargo run --release

doc: ## Generate and open documentation
	cargo doc --open

bench: ## Run benchmarks (if any)
	cargo bench

all: fmt lint test build ## Format, lint, test, and build

ci: fmt-check lint test ## Run CI checks

install-tools: ## Install development tools
	rustup component add rustfmt clippy
	cargo install cargo-watch

reset-db: ## Reset database (delete proyects.db and restart with seed data)
	rm -f proyects.db proyects.db-shm proyects.db-wal
	@echo "Database deleted. Run 'make run' to recreate with seed data."

reset-db-empty: ## Reset database without seed data
	rm -f proyects.db proyects.db-shm proyects.db-wal
	rm -f migrations/20250107000000_seed_test_data.sql
	@echo "Database deleted and seed migration removed. Run 'make run' to recreate empty database."

.DEFAULT_GOAL := help
