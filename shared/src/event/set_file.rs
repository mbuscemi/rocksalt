use serde::{ Serialize, Deserialize };

use super::Detail;
use super::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetFile {
    pub contents: String,
}

impl Detail for SetFile {
    const NAME: &'static str = "setfile";

    fn transform(&self) -> Message {
        Message::SetFile(self.contents.clone())
    }
}
