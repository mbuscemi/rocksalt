clean:
	rm -rf frontend/pkg

process:
	cd frontend && wasm-pack build --target no-modules
	cd shared && cargo test

build:
	cargo build

run:
	cargo run
