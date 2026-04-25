pub mod set_file;
pub mod set_project_path;

use crate::message::YewMessage;
use serde::Serialize;

pub trait Detail {
    fn name() -> String;
    fn transform(&self) -> YewMessage;
}

pub fn command_for_webview<D>(detail: D) -> String
    where D: Detail + Serialize
{
    format!(
        r#"document.dispatchEvent(
            new CustomEvent("{}", {{ detail: {} }})
        );"#,
        D::name(),
        serde_json::to_string(&detail).expect("failed to JSON encode detail object"),
    )
}
