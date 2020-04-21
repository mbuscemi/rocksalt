use crate::file_system::disk_entry::DiskEntry;

pub enum Message {
    SelectFile,
    OpenFile(String),
    SetFile(String),
    OpenProject,
    SetProjectPath(String, Vec<DiskEntry>),
    CloseProject,
    ToggleHierarchy(String),
    Noop,
}
