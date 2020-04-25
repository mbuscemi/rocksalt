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
    fn name() -> String { "setprojectpath".to_string() }

    fn transform(&self) -> YewMessage {
        YewMessage::SetProjectPath(self.path.clone(), self.dir_structure.clone())
    }
}
