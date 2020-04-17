clean:
	rm static/rocksalt-frontend.js
	rm static/rocksalt-frontend.wasm

process:
	cd frontend && cargo web build --release
	cp frontend/target/wasm32-unknown-unknown/release/rocksalt-frontend.js static/
	cp frontend/target/wasm32-unknown-unknown/release/rocksalt-frontend.wasm static/

build:
	cargo build

run:
	cargo run
