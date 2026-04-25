# Frontend Modernization Plan

## Context

Rocksalt is a desktop editor built with Rust. It has two compiled artifacts:

1. **Native binary** (`rocksalt`) — built with `cargo build` from the project root. Uses `web-view 0.6.3` to display an embedded browser window. Reads pre-compiled WASM/JS from `static/` at compile time via `include_bytes!`/`include_str!`.

2. **Frontend WASM** (`rocksalt_frontend`) — the browser-side UI, a Yew application compiled to WebAssembly. Source is in `frontend/`. The compiled output (`static/rocksalt_frontend.wasm`, `static/rocksalt_frontend.js`) is committed to git.

## Why modernization is needed

The frontend currently uses `yew-stdweb 0.15` and `stdweb 0.4.20`, both abandoned around 2019–2020. Building the frontend from source requires `cargo-web`, which:
- Panics on `cargo metadata` output from Cargo 1.73+ (the `workspace_members` format changed from `"name version (url)"` to `"url#name@version"`)
- Is incompatible with modern `wasm-bindgen` (0.2.118 requires Rust 1.77+, but cargo-web needs Cargo ≤ 1.72)

The goal is to replace this stack with `wasm-pack` + modern `yew` (0.21+) so the frontend can be built with the current Rust toolchain.

## Crate structure (as of April 2026)

```
rocksalt/                  (workspace root, native binary)
├── src/
│   ├── main.rs            — web_view::builder() entry point
│   ├── event_handler.rs   — handles messages from the webview (SelectFile, OpenFile, SelectProject)
│   ├── rpc.rs             — dispatches JS CustomEvents into the webview
│   └── inline_code.rs     — embeds static/ assets into HTML at compile time
├── shared/                (library shared between native and frontend)
│   └── src/
│       ├── lib.rs
│       ├── message.rs     — WebviewMessage, YewMessage enums
│       ├── utils.rs       — on_some, on_ok helpers
│       ├── event/
│       │   ├── mod.rs     — Detail trait + command_for_webview() free fn
│       │   ├── set_file.rs
│       │   └── set_project_path.rs
│       └── file_system/   — file types, disk entries, path utilities
└── frontend/              (WASM library — the Yew UI)
    └── src/
        ├── lib.rs         — Component impl for Model
        ├── main.rs        — yew::start_app entry point
        ├── model.rs       — Model struct
        ├── view.rs        — HTML rendering
        └── event.rs       — Event struct (DOM event listeners, js! macros)
```

## What shared/ exports that frontend uses

- `rocksalt_shared::message::{WebviewMessage, YewMessage}` — message enums
- `rocksalt_shared::event::{Detail, command_for_webview}` — the `Detail` trait (implemented by event detail types); `command_for_webview` is used by the native side only
- `rocksalt_shared::event::set_file::SetFile<F>` — event detail type
- `rocksalt_shared::event::set_project_path::SetProjectPath` — event detail type
- `rocksalt_shared::file_system::*` — file types, DiskEntry, FileType

## What the frontend currently does (old stack)

`frontend/src/event.rs` (currently uses stdweb):
- `Event` struct holds `stdweb::Value` (a JS object reference)
- `Event::create_for_yew` — registers a DOM CustomEvent listener, deserializes the payload, calls a Yew callback
- `Event::destroy_for_yew` — removes the DOM listener
- `Event::invoke_on_webview` — calls `external.invoke(json)` (the web-view RPC channel)

`frontend/src/lib.rs` — implements `yew::Component` for `Model`:
- `create`: registers 3 events (SetFile<PlainText>, SetFile<CobaltMarkdown>, SetProjectPath)
- `update`: handles YewMessage variants, calls Event::invoke_on_webview for outbound messages
- `view`: renders the UI

## The modernization task

Replace `yew-stdweb 0.15` + `stdweb 0.4.20` with `yew 0.21` + `wasm-bindgen` + `wasm-pack`.

### Step 1 — Update `frontend/Cargo.toml`

Remove:
```toml
yew = { version = "0.15.0", package = "yew-stdweb" }
stdweb = "0.4.20"
```

Add:
```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "Window", "Document", "EventTarget", "CustomEvent",
    "CustomEventInit", "Event",
] }
js-sys = "0.3"
serde-wasm-bindgen = "0.6"
gloo = { version = "0.11", features = ["events"] }
```

Also add a `[lib]` section:
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

### Step 2 — Remove `frontend/.cargo/config.toml`

Delete the file. The `cargo_web` rustflag and `wasm32-unknown-unknown` default target are no longer needed — `wasm-pack` handles targeting.

### Step 3 — Rewrite `frontend/src/event.rs`

The new `Event` struct replaces stdweb's `Value` and `js!` macro with `web-sys` and `gloo::events`.

```rust
use gloo::events::EventListener;
use rocksalt_shared::event::Detail;
use rocksalt_shared::message::{WebviewMessage, YewMessage};
use serde::Deserialize;
use wasm_bindgen::JsValue;
use web_sys::CustomEvent;
use yew::Callback;

pub struct Event {
    _listener: EventListener,  // kept alive for the lifetime of the Event
}

impl Event {
    pub fn create_for_yew<D>(callback: Callback<YewMessage>) -> Self
    where
        D: 'static + Detail + for<'de> Deserialize<'de>,
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let listener = EventListener::new(&document, &D::name(), move |event| {
            let custom_event = event.dyn_ref::<CustomEvent>().unwrap();
            let detail: D = serde_wasm_bindgen::from_value(custom_event.detail()).unwrap();
            callback.emit(detail.transform());
        });
        Event { _listener: listener }
    }

    pub fn invoke_on_webview(message: WebviewMessage) {
        let json = serde_json::to_string(&message).unwrap();
        // web-view's JS bridge: external.invoke(json_string)
        let js = format!("window.external.invoke({})", json);
        js_sys::eval(&js).unwrap();
    }
}
```

Notes:
- `EventListener` from gloo automatically removes the DOM listener when dropped — no explicit `destroy_for_yew` needed.
- The `_listener` field keeps it alive as long as the `Event` is alive.
- `serde-wasm-bindgen` replaces `stdweb::serde::Serde` for deserializing the CustomEvent detail.

### Step 4 — Update `frontend/src/lib.rs`

Remove:
```rust
#[macro_use]
extern crate stdweb;
```

Update the `Component` impl to match Yew 0.21's API. Yew 0.21 uses function components and hooks by default, but class-based components still exist via `yew::Component`. The key API differences:

**Old (0.15):**
```rust
fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self { ... }
fn update(&mut self, msg: Self::Message) -> ShouldRender { ... }
fn change(&mut self, _: Self::Properties) -> bool { false }
fn view(&self) -> Html { ... }
```

**New (0.21):**
```rust
fn create(ctx: &Context<Self>) -> Self { ... }
fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool { ... }
fn view(&self, ctx: &Context<Self>) -> Html { ... }
// changed() is now optional, no longer requires implementation
```

The `link` is accessed via `ctx.link()`. Callbacks are created with `ctx.link().callback(...)`.

The `events` array becomes `Vec<Event>` (no fixed size needed since gloo listeners auto-drop).

### Step 5 — Update `frontend/src/model.rs`

Remove the `Event` import. The `Model` struct's `events` field changes from `[Event; 3]` to `Vec<Event>` (or can hold them as named fields). The `ComponentLink` type is removed.

**Old:**
```rust
use rocksalt_shared::event::Event;  // was here before our changes
use crate::event::Event;            // after our changes this session
use yew::ComponentLink;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub events: [Event; 3],
    ...
}
```

**New:**
```rust
use crate::event::Event;

pub struct Model {
    pub events: Vec<Event>,
    ...
}
```

### Step 6 — Update `frontend/src/view.rs`

Check for any stdweb-specific imports or patterns. The `html!` macro in Yew 0.21 has some syntax changes:
- Callback bindings: `onclick={ctx.link().callback(...)}` instead of `onclick=...`
- Read the current file to understand what needs updating.

### Step 7 — Update the build workflow

Old: `cargo web build` (from `frontend/` directory)
New: `wasm-pack build --target web` (from `frontend/` directory)

This produces output in `frontend/pkg/`. The relevant files are:
- `frontend/pkg/rocksalt_frontend.wasm`
- `frontend/pkg/rocksalt_frontend.js` (the JS glue code)

### Step 8 — Update `src/inline_code.rs`

The wasm-pack output format differs from cargo-web output. The JS glue code has different structure. The `rocksalt_frontend.js` from wasm-pack uses ES module syntax and initializes differently.

The `replace_yew_wasm_with_inline_wasm` function currently replaces cargo-web's WASM loader with an inline loader. This will need to be updated to match wasm-pack's initialization pattern.

The wasm-pack JS file typically exports an `init` function. The inline HTML needs to call `init(wasmCode)` after inlining the WASM bytes.

Key change: update `static/` path references in `inline_code.rs` to point to `frontend/pkg/` instead, or copy the wasm-pack output to `static/` as part of the build process.

### Step 9 — Update `shared/Cargo.toml` (optional cleanup)

Remove the version pins that were added to work around old toolchain issues:
```toml
ignore = ">=0.4.14, <0.4.24"   # can revert to "0.4.14"
globset = "=0.4.15"             # can remove entirely (it's a transitive dep)
```

### Step 10 — Remove `frontend/` from `[dev-dependencies]` in root `Cargo.toml`

```toml
# Remove this:
[dev-dependencies]
rocksalt_frontend = { path = "frontend" }
```

The workspace declaration handles the relationship now. The dev-dependency was a workaround anyway.

## Key things to verify after modernization

1. `cargo build` from root still compiles cleanly
2. `wasm-pack build --target web` from `frontend/` succeeds
3. The generated `rocksalt_frontend.js` and `.wasm` can be read by `inline_code.rs` at compile time
4. The app runs — open/close files, project tree, keyboard shortcuts

## Files to read at the start of the new session

Read these files to understand the current state before making changes:
- `frontend/src/lib.rs`
- `frontend/src/model.rs`
- `frontend/src/view.rs`
- `frontend/src/event.rs`
- `frontend/Cargo.toml`
- `shared/src/event/mod.rs`
- `shared/src/message.rs`
- `src/inline_code.rs`
- `Cargo.toml` (workspace root)
