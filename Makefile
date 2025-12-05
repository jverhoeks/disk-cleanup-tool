.PHONY: help build test clean release check install run

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

check: ## Run pre-release checks
	./scripts/pre-release-check.sh

clean: ## Clean build artifacts
	cargo clean

install: build ## Install the binary to ~/.cargo/bin
	cargo install --path .

run: ## Run the tool (use ARGS="--path ~/projects" to pass arguments)
	cargo run --release -- $(ARGS)

release: check ## Create a GitHub release
	./scripts/quick-release.sh

fmt: ## Format code
	cargo fmt

lint: ## Run clippy lints
	cargo clippy -- -D warnings

dev: ## Run in development mode with debug output
	RUST_LOG=debug cargo run -- $(ARGS)

watch: ## Watch for changes and run tests
	cargo watch -x test

.DEFAULT_GOAL := help
