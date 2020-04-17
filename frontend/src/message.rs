pub enum Message {
    OpenFile,
    SetFile(String),
    OpenProject,
    SetProjectPath(String),
    CloseProject,
}
