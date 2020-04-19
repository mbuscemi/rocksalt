use rocksalt_shared::disk_entry::DiskEntry;

pub enum Message {
    OpenFile,
    SetFile(String),
    OpenProject,
    SetProjectPath(String, Vec<DiskEntry>),
    CloseProject,
    ToggleHierarchy(String),
}
