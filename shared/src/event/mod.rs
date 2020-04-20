pub mod set_file;
pub mod set_project_path;

use crate::message::Message;
use serde::{ Serialize, Deserialize };
use stdweb::{ serde::Serde, unstable::TryInto, Value };
use yew::{ Component, ComponentLink };

pub trait Detail {
    const NAME: &'static str;
    fn transform(&self) -> Message;
}

pub struct Event {
    pub yew_js_refs: Value,
}

impl<'a> Event {
    pub fn create_for_yew<C, D>(link: &'a ComponentLink<C>) -> Self
        where C: Component,
              <C as Component>::Message: From<Message>,
              D: 'static + Detail + Deserialize<'a>
    {
        let yew_callback = link.callback(|detail: D| detail.transform() );

        let js_callback = move |value: Value| {
            let structure: Serde<D> = value.try_into().expect(&format!("unable to parse payload from event: {}", D::NAME));
            let detail: D = structure.0;
            yew_callback.emit(detail)
        };

        let js_refs =
            js! {
                var callback = @{js_callback};
                var listener = event => callback(event.detail);
                document.addEventListener(@{D::NAME}, listener);
                return {
                    name: @{D::NAME},
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
            D::NAME,
            serde_json::to_string(&detail).expect("failed to JSON encode detail object"),
        )
    }
}
