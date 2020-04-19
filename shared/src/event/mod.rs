pub mod message;
pub mod set_file;
pub mod set_project_path;

use message::Message;
use serde::de::Deserialize;
use stdweb::{ serde::Serde, unstable::TryInto, Value };
use yew::{ Component, ComponentLink };

pub trait Detail {
    const NAME: &'static str;
    fn transform(&self) -> Message;
}

pub struct Event {
    pub callback: Value,
}

impl<'a> Event {
    pub fn new<C, D>(link: &'a ComponentLink<C>) -> Self
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

        let callback =
            js! {
                var callback = @{js_callback};
                document.addEventListener(@{D::NAME}, event => callback(event.detail));
                return callback;
            };

        Event { callback: callback }
    }

    pub fn destroy(&self) {
        let callback = &self.callback;
        js! {
            var callback = @{callback};
            callback.drop();
        }
    }
}
