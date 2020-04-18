use stdweb::Value;
use yew::ComponentLink;

use crate::event::JsRegistration;
use crate::message::Message;
use crate::model::Model;

pub struct SetFileEvent {
    callback: Value,
}

impl Default for SetFileEvent {
    fn default() -> Self {
        Self{ callback: js! {} }
    }
}

impl JsRegistration for SetFileEvent {
    fn setup(&mut self, link: &ComponentLink<Model>) {
        let callback = link.callback(|content: String| Message::SetFile(content));

        let js_callback = move |value: Value| {
            callback.emit(
                value
                    .into_string()
                    .expect("unable to parse payload from setfile")
            )
        };

        self.callback =
            js! {
                var callback = @{js_callback};
                document.addEventListener("setfile", event => callback(event.detail.contents));
                return callback;
            };
    }

    fn destroy(&self) {
        let callback = &self.callback;
        js! {
            var callback = @{callback};
            callback.drop();
        }
    }
}
