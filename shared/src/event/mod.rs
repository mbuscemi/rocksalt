pub mod set_file;
pub mod set_project_path;

use crate::message::{ WebviewMessage, YewMessage };
use serde::{ Serialize, Deserialize };
use stdweb::{ serde::Serde, unstable::TryInto, Value };
use yew::{ Component, ComponentLink };

pub trait Detail {
    fn name() -> String;
    fn transform(&self) -> YewMessage;
}

pub struct Event {
    pub yew_js_refs: Value,
}

impl<'a> Event {
    pub fn create_for_yew<C, D>(link: &'a ComponentLink<C>) -> Self
        where C: Component,
              <C as Component>::Message: From<YewMessage>,
              D: 'static + Detail + Deserialize<'a>
    {
        let yew_callback = link.callback(|detail: D| detail.transform() );

        let js_callback = move |value: Value| {
            let structure: Serde<D> = value.try_into().expect(&format!("unable to parse payload from event: {}", D::name()));
            let detail: D = structure.0;
            yew_callback.emit(detail)
        };

        let js_refs =
            js! {
                var callback = @{js_callback};
                var listener = event => callback(event.detail);
                document.addEventListener(@{D::name()}, listener);
                return {
                    name: @{D::name()},
                    callback: callback,
                    listener: listener
                };
            };

        Event { yew_js_refs: js_refs }
    }

    pub fn destroy_for_yew(&self) {
        js! {
            var refs = @{&self.yew_js_refs};
            document.removeEventListener(refs.name, refs.listener);
            refs.callback.drop();
        }
    }

    pub fn command_for_webview<D>(detail: D) -> String
        where D: Detail + Serialize
    {
        format!(
            r#"document.dispatchEvent(
                new CustomEvent("{}", {{ detail: {} }})
            );"#,
            D::name(),
            serde_json::to_string(&detail).expect("failed to JSON encode detail object"),
        )
    }

    pub fn invoke_on_webview(message: WebviewMessage) {
        let json_message = serde_json::to_string(&message).expect("failed to JSON encode WebviewMessage");
        js! { external.invoke(@{json_message}); }
    }
}
