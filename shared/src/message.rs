use serde::{ Serialize, Deserialize };

use crate::file_system::disk_entry::DiskEntry;

#[derive(Serialize, Deserialize)]
#[serde(tag = "msg")]
pub enum WebviewMessage {
    SelectFile,
    OpenFile { path: String },
    SelectProject,
}

pub enum YewMessage {
    SelectFile,
    OpenFile(String),
    SetFile(String),
    OpenProject,
    SetProjectPath(String, Vec<DiskEntry>),
    CloseProject,
    ToggleHierarchy(String),
    Noop,
}
