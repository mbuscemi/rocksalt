use serde::Deserialize;

use super::Detail;
use super::message::Message;

#[derive(Deserialize, Debug)]
pub struct SetFile {
    contents: String,
}

impl Detail for SetFile {
    const NAME: &'static str = "setfile";

    fn transform(&self) -> Message {
        Message::SetFile(self.contents.clone())
    }
}
