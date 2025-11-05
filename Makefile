all: check test

check: fmt lint

fmt:
	@echo "--- Formatting code... ---"
	cargo fmt

lint:
	@echo "--- Linting code... ---"
	cargo clippy -- -D warnings

test:
	@echo "--- Running tests... ---"
	cargo test

run:
	@echo "--- Running parser... ---"
	@cargo run -- parse $(file)

commit-check: check test

build:
	cargo build --release


.PHONY: all check fmt lint test run commit-check build