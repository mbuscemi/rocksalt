use serde::{ Serialize, Deserialize };

use super::Detail;
use crate::message::YewMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetFile {
    pub contents: String,
}

impl Detail for SetFile {
    const NAME: &'static str = "setfile";

    fn transform(&self) -> YewMessage {
        YewMessage::SetFile(self.contents.clone())
    }
}
