test:
	cargo test --locked --bins --lib

lint:
	cargo clippy --locked -- -D warnings

watch-tests:
	cargo watch -x "test --locked --bins --lib"
