test:
	cargo test --locked --quiet --lib --workspace --release

lint:
	cargo clippy --locked --workspace -- -D warnings

watch-tests:
	cargo watch -x "test --locked --bins --lib --workspace"
