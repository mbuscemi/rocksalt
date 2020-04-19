use serde::Deserialize;

use super::Detail;
use crate::file_system::disk_entry::DiskEntry;
use super::message::Message;

#[derive(Deserialize, Debug)]
pub struct SetProjectPath {
    path: String,
    dir_structure: Vec<DiskEntry>,
}

impl Detail for SetProjectPath {
    const NAME: &'static str = "setprojectpath";

    fn transform(&self) -> Message {
        Message::SetProjectPath(self.path.clone(), self.dir_structure.clone())
    }
}
