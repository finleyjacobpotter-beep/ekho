
build: target/debug/cross-rust-overlay

target/debug/cross-rust-overlay:
	cargo b

clean:
	cargo clean

lint:
	cargo clippy

test:
	cargo test
