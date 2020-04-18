use serde::Deserialize;

use crate::event::Detail;
use crate::message::Message;

#[derive(Deserialize, Debug)]
pub struct SetProjectPath {
    path: String,
}

impl Detail<Message> for SetProjectPath {
    fn transform(&self) -> Message {
        Message::SetProjectPath(self.path.clone())
    }
}
