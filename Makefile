clean:
	rm static/rocksalt_frontend.js
	rm static/rocksalt_frontend.wasm

process:
	cd frontend && cargo web build --release
	cp frontend/target/wasm32-unknown-unknown/release/rocksalt_frontend.js static/
	cp frontend/target/wasm32-unknown-unknown/release/rocksalt_frontend.wasm static/

build:
	cargo build

run:
	cargo run
