# quantumseal — build & workflow automation
#
# Cross-platform-ish Makefile. Rust uses cargo; the TypeScript viewer uses npm.
# The `demo` target runs the full pipeline: scan fixtures -> CryptoBOM -> render.

CARGO ?= cargo
NPM ?= npm
VIEWER_DIR := viewer
EXAMPLES_DIR := examples

.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help
	@echo "quantumseal targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  %-16s %s\n", $$1, $$2}'

# ---- Rust ------------------------------------------------------------------

.PHONY: build
build: ## Build the Rust CLI (release)
	$(CARGO) build --release

.PHONY: test
test: ## Run all Rust tests
	$(CARGO) test

.PHONY: fmt
fmt: ## Format Rust sources
	$(CARGO) fmt

.PHONY: fmt-check
fmt-check: ## Check Rust formatting
	$(CARGO) fmt --check

