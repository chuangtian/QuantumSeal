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

.PHONY: clippy
clippy: ## Run clippy if available (non-fatal if missing)
	-$(CARGO) clippy --all-targets -- -D warnings

# ---- TypeScript viewer -----------------------------------------------------

.PHONY: viewer-install
viewer-install: ## Install viewer dev dependencies
	cd $(VIEWER_DIR) && $(NPM) install

.PHONY: viewer-build
viewer-build: ## Type-check and compile the viewer
	cd $(VIEWER_DIR) && $(NPM) run build

.PHONY: viewer-test
viewer-test: viewer-build ## Run viewer tests
	cd $(VIEWER_DIR) && $(NPM) test

.PHONY: viewer-typecheck
viewer-typecheck: ## Type-check the viewer without emitting
	cd $(VIEWER_DIR) && $(NPM) run typecheck

# ---- Combined --------------------------------------------------------------

.PHONY: all
all: build test viewer-build viewer-test ## Build and test everything

.PHONY: demo
demo: build viewer-build ## Run the end-to-end demo against the fixtures
	./target/release/quantumseal scan fixtures/legacy_service --format json --output $(EXAMPLES_DIR)/legacy-cbom.json
	./target/release/quantumseal scan fixtures/migrated_service --format json --output $(EXAMPLES_DIR)/migrated-cbom.json
	node $(VIEWER_DIR)/dist/cli.js $(EXAMPLES_DIR)/legacy-cbom.json --format html --out $(EXAMPLES_DIR)/legacy-report.html
	node $(VIEWER_DIR)/dist/cli.js $(EXAMPLES_DIR)/legacy-cbom.json --format markdown --out $(EXAMPLES_DIR)/legacy-report.md
	./target/release/quantumseal diff $(EXAMPLES_DIR)/legacy-cbom.json --path fixtures/migrated_service

.PHONY: clean
clean: ## Remove build artifacts
	$(CARGO) clean
	cd $(VIEWER_DIR) && $(NPM) run clean
