use serde::{ Serialize, Deserialize };

use super::Detail;
use crate::file_system::disk_entry::DiskEntry;
use crate::message::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetProjectPath {
    pub path: String,
    pub dir_structure: Vec<DiskEntry>,
}

impl Detail for SetProjectPath {
    const NAME: &'static str = "setprojectpath";

    fn transform(&self) -> Message {
        Message::SetProjectPath(self.path.clone(), self.dir_structure.clone())
    }
}
