use serde::{ Serialize, Deserialize };

use crate::file_system::{ disk_entry::DiskEntry, file::File, file_type::FileType };

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
    SetFile(Box<dyn File>),
    UnsetFile,
    OpenProject,
    SetProjectPath(String, Vec<DiskEntry>),
    CloseProject,
    ToggleHierarchy(String),
    Noop,
}
