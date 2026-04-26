use gloo::events::EventListener;
use rocksalt_shared::event::Detail;
use rocksalt_shared::message::{WebviewMessage, YewMessage};
use serde::Deserialize;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use yew::Callback;

pub struct Event {
    _listener: EventListener,
}

impl Event {
    pub fn create_for_yew<D>(callback: Callback<YewMessage>) -> Self
    where
        D: 'static + Detail + for<'de> Deserialize<'de>,
    {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let listener = EventListener::new(&document, D::name(), move |event| {
            let custom_event = event.dyn_ref::<CustomEvent>().unwrap();
            let detail: D = serde_wasm_bindgen::from_value(custom_event.detail()).unwrap();
            callback.emit(detail.transform());
        });
        Event { _listener: listener }
    }

    pub fn invoke_on_webview(message: WebviewMessage) {
        let json = serde_json::to_string(&message).unwrap();
        // wry IPC requires a string argument; double-serialize so the JS value is a string literal
        let js_string = serde_json::to_string(&json).unwrap();
        let js = format!("window.ipc.postMessage({})", js_string);
        js_sys::eval(&js).unwrap();
    }
}
