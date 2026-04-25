# rocksalt
an editor and productivity tool for Cobalt projects

---

## 2026 changes

This project was originally written in 2019–2020 and built on Ubuntu. In April 2026 it was revived on Windows 11 with a modern Rust toolchain. This section documents what broke, why, and what was changed to restore a working build.

### Build architecture

The project has two independently compiled artifacts:

- **Native binary** (`rocksalt`) — a Windows desktop app built with `cargo build`. Uses `web-view` to embed an IE-based browser window (MSHTML backend). The frontend WASM/JS assets are embedded directly into the binary at compile time via `include_bytes!`/`include_str!` in `src/inline_code.rs`.
- **Frontend WASM** (`rocksalt_frontend`) — the browser-side UI written in Yew, compiled to WebAssembly. The compiled `.wasm` and `.js` files live in `static/` and are committed to git.

Building the native binary does NOT rebuild the frontend. The frontend is rebuilt separately (see below) and the output committed to `static/`.

### What was broken on Windows

#### 1. `yew-stdweb` and `stdweb` in the shared library

`rocksalt_shared` was depending on `yew-stdweb 0.15` and `stdweb 0.4.20` directly, even though those crates are WebAssembly-only. On Linux this worked silently because GNU ld skips object files from archives that are not needed. On Windows, MSVC `link.exe` is stricter and reports every unresolved external symbol it encounters — including `emscripten_asm_const_int`, `emscripten_pause_main_loop`, and `emscripten_set_main_loop`, which are Emscripten runtime functions that only exist in a WASM/Emscripten toolchain, not in a native Windows build.

**Fix:** Removed `yew` and `stdweb` from `shared/Cargo.toml`. Moved the `Event` struct (which holds stdweb types and uses `js!` macros) from `shared/src/event/mod.rs` into a new file `frontend/src/event.rs`. The `command_for_webview` function (the only part of `Event` used by the native binary) was promoted to a plain free function in shared. Updated `frontend/src/lib.rs` to add `#[macro_use] extern crate stdweb` and import `Event` from the local module. Updated `src/rpc.rs` to call the free function instead of an associated function on `Event`.

#### 2. Missing `advapi32.lib` in `webview-sys`

`web-view 0.6.x` uses `webview-sys` on Windows, which calls Windows Registry APIs (`RegCloseKey`, `RegCreateKeyW`, `RegSetValueExW`) to set an IE compatibility mode. The `webview-sys` build script links `ole32`, `comctl32`, `oleaut32`, `uuid`, `gdi32`, and `user32` — but omits `advapi32`, which is where the Registry functions live.

**Fix:** Added a `build.rs` to the root crate that emits `cargo:rustc-link-lib=advapi32` on Windows.

### State of the frontend build (April 2026)

The frontend currently uses `yew-stdweb 0.15` + `stdweb 0.4.20`, which required `cargo-web` as the build tool. `cargo-web` is abandoned (last released 2019) and cannot be used with Cargo 1.73 or newer because Cargo changed the `workspace_members` format in its metadata output (from `"name version (url)"` to `"url#name@version"`), which causes `cargo-web`'s internal metadata parser to panic.

The pre-compiled frontend assets in `static/` are functional. The native binary builds and runs. However, rebuilding the frontend from source is currently not possible with a modern toolchain.

A modernization plan exists at `.claude/frontend-modernization-plan.md`. It covers replacing `yew-stdweb` + `stdweb` + `cargo-web` with `yew 0.21` + `wasm-bindgen` + `wasm-pack`.

### Toolchain configuration

The project root now has a `rustup` override set to **Rust 1.77** (`rustup override set 1.77` was run in the project directory). This was necessary because:
- Rust 1.77 is the latest version where `wasm-bindgen-shared` resolves to a version compatible with our dependency constraints (though the frontend is still not fully buildable — see above).
- Rust 1.77 builds the native binary cleanly.

To remove the override and use stable: `rustup override unset` from the project root.

### Cargo workspace

A `[workspace]` section was added to the root `Cargo.toml` with `members = ["shared", "frontend"]` and `default-members = ["."]`. The `default-members` setting ensures that `cargo build` from the root only builds the native binary by default and does not attempt to compile the WASM-only frontend for a native target.

### Dependency version pins

Two transitive dependencies were pinned in `shared/Cargo.toml` to stay below the versions that adopted edition 2024 (which requires Rust 1.85+):

- `ignore` pinned to `>=0.4.14, <0.4.24`
- `globset` pinned to `=0.4.15`

These pins can be removed once the frontend is modernized and the project no longer needs to support Rust 1.77.

### How to build

**Native binary (works today):**
```
cargo build
```

**Frontend (requires modernization — see `.claude/frontend-modernization-plan.md`):**
```
# Not currently possible with modern tooling.
# The pre-built assets in static/ are committed to git.
```
