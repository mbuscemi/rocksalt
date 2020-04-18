use stdweb::Value;
use yew::ComponentLink;

use crate::event::JsRegistration;
use crate::message::Message;
use crate::model::Model;

pub struct SetProjectPathEvent {
    callback: Value,
}

impl Default for SetProjectPathEvent {
    fn default() -> Self {
        Self{ callback: js! {} }
    }
}

impl JsRegistration for SetProjectPathEvent {
    fn setup(&mut self, link: &ComponentLink<Model>) {
        let callback = link.callback(|path: String| Message::SetProjectPath(path));

        let js_callback = move |value: Value| {
            callback.emit(
                value
                    .into_string()
                    .expect("unable to parse payload from setprojectpath")
            )
        };

        self.callback =
            js! {
                var callback = @{js_callback};
                document.addEventListener("setprojectpath", event => callback(event.detail.path));
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
