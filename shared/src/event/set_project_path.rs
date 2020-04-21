use serde::{ Serialize, Deserialize };

use super::Detail;
use crate::file_system::disk_entry::DiskEntry;
use crate::message::YewMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetProjectPath {
    pub path: String,
    pub dir_structure: Vec<DiskEntry>,
}

impl Detail for SetProjectPath {
    const NAME: &'static str = "setprojectpath";

    fn transform(&self) -> YewMessage {
        YewMessage::SetProjectPath(self.path.clone(), self.dir_structure.clone())
    }
}
