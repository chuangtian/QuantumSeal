# quantumseal — build & workflow automation
#
# Cross-platform-ish Makefile. Rust uses cargo; the TypeScript viewer uses npm.
# The `demo` target runs the full pipeline: scan fixtures -> CryptoBOM -> render.

CARGO ?= cargo
NPM ?= npm
