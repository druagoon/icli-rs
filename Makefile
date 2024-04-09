SHELL := bash

.PHONY: clean
clean:
	@if [[ -d ./target ]]; then \
		rm -rf ./target; \
	fi

.PHONY: check
check:
	@cargo +nightly fmt --check

.PHONY: fmt
fmt:
	@cargo +nightly fmt

.PHONY: build
build: fmt
	@cargo build

.PHONY: release
release: fmt
	@cargo build --release

.PHONY: run
run: fmt
	@cargo run
