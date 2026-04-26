# rocksalt
an editor and productivity tool for Cobalt projects

---

## Building and Running

The project has two independently compiled artifacts that must be built in order:

**1. Build the frontend (WebAssembly):**
```
make process
```
This compiles the Yew frontend to WASM using `wasm-pack` and runs the shared library tests. Output lands in `frontend/pkg/`.

**2. Build the native binary:**
```
make build
```
This compiles the desktop app with `cargo build`. The frontend assets from `frontend/pkg/` are embedded into the binary at compile time.

**Run:**
```
make run
```

**Clean frontend build artifacts:**
```
make clean
```

> The frontend must be built before the native binary. The `frontend/pkg/` output is committed to git, so `make build` works out of the box without running `make process` first unless you've changed the frontend.

---

## 2026 Changes

This project was originally written in 2019–2020. In April 2026, it was revived on Windows 11 with a modern Rust toolchain. Two significant upgrades were made.

### Native binary: Windows build restored

The native binary (`rocksalt`) uses `web-view` to display an embedded browser window. Getting it to build on Windows required fixing linker issues caused by WebAssembly-only dependencies that had leaked into the shared library, and adding a missing `advapi32` link directive for the Windows Registry APIs used by `web-view`. A `build.rs` was added to the root crate to handle the latter.

### Frontend: modernized to wasm-pack + Yew 0.21

The frontend previously used `yew-stdweb 0.15`, `stdweb 0.4.20`, and `cargo-web` — all abandoned since 2019 and incompatible with Cargo 1.73 and newer. The entire frontend stack was replaced:

| Before | After |
|---|---|
| `yew-stdweb 0.15` | `yew 0.21` |
| `stdweb 0.4.20` | `wasm-bindgen` + `web-sys` + `gloo` |
| `cargo web build` | `wasm-pack build --target no-modules` |
| output in `static/` | output in `frontend/pkg/` |

The frontend can now be built from source with any modern Rust installation. The `#[wasm_bindgen(start)]` entry point wires the Yew renderer to the wasm-pack initialization lifecycle. The native binary's HTML template was updated to use `wasm_bindgen.initSync()` rather than the old cargo-web loader.
