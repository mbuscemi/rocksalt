pub mod set_file;
pub mod set_project_path;

use serde::de::Deserialize;
use stdweb::{ serde::Serde, unstable::TryInto, Value };
use yew::ComponentLink;

use crate::message::Message;
use crate::model::Model;

pub trait Detail {
    const NAME: &'static str;
    fn transform(&self) -> Message;
}

pub struct Event {
    pub callback: Value,
}

impl<'a> Event {
    pub fn new<D: 'static + Detail + Deserialize<'a>>(link: &'a ComponentLink<Model>) -> Self {
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
