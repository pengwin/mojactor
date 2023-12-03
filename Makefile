.PHONY: build clean lint test run

run: lint
	cargo run -p test-bench

run-release: lint
	cargo run -p test-bench --release

build: lint
	cargo build

clean:
	cargo clean

lint:
	cargo clippy --all-features -- -D clippy::unwrap_used -W missing_docs -W clippy::pedantic -W clippy::cargo -A clippy::cargo_common_metadata -A clippy::module_name_repetitions

test: lint
	cargo test --all-features

bench: lint
	cargo bench