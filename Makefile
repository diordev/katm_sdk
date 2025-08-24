docs:
	rm -rf target/doc && cargo doc --no-deps --open

test:
	cargo test --tests
