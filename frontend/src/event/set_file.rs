use serde::Deserialize;

use crate::event::Detail;
use crate::message::Message;

#[derive(Deserialize, Debug)]
pub struct SetFile {
    contents: String,
}

impl Detail<Message> for SetFile {
    const NAME: &'static str = "setfile";

    fn transform(&self) -> Message {
        Message::SetFile(self.contents.clone())
    }
}
