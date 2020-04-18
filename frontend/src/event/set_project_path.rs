use serde::Deserialize;

use crate::disk_entry::DiskEntry;
use crate::event::Detail;
use crate::message::Message;

#[derive(Deserialize, Debug)]
pub struct SetProjectPath {
    path: String,
    dir_structure: Vec<DiskEntry>,
}

impl Detail<Message> for SetProjectPath {
    fn transform(&self) -> Message {
        Message::SetProjectPath(self.path.clone(), self.dir_structure.clone())
    }
}
