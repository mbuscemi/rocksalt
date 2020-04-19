use crate::file_system::disk_entry::DiskEntry;

pub enum Message {
    OpenFile,
    SetFile(String),
    OpenProject,
    SetProjectPath(String, Vec<DiskEntry>),
    CloseProject,
    ToggleHierarchy(String),
    Noop,
}
