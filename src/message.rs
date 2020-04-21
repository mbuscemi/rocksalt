#[derive(Deserialize)]
#[serde(tag = "msg")]
pub enum Message {
    SelectFile,
    OpenFile,
    SelectProject,
}
