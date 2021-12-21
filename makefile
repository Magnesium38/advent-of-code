test:
	cargo test --locked --bins --lib --workspace

lint:
	cargo clippy --locked --workspace -- -D warnings

watch-tests:
	cargo watch -x "test --locked --bins --lib --workspace"
