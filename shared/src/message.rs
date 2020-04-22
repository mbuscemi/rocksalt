use serde::{ Serialize, Deserialize };

use crate::file_system::{ disk_entry::DiskEntry, file_type::FileType };

#[derive(Serialize, Deserialize)]
#[serde(tag = "msg")]
pub enum WebviewMessage {
    SelectFile,
    OpenFile { path: String, file_type: FileType },
    SelectProject,
}

pub enum YewMessage {
    SelectFile,
    OpenFile { path: String, file_type: FileType },
    SetFile(String),
    OpenProject,
    SetProjectPath(String, Vec<DiskEntry>),
    CloseProject,
    ToggleHierarchy(String),
    Noop,
}
