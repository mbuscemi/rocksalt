use rocksalt_shared::file_system::disk_entry::DiskEntry;
use rustc_serialize::json::Json;
use web_view::WebView;

pub fn set_file(webview: &mut WebView<()>, contents: String) {
    let cmd = format!(
        r#"document.dispatchEvent(
            new CustomEvent("setfile", {{ detail: {{ contents: {} }} }})
        );"#,
        Json::String(contents),
    );
    webview.eval(&cmd).expect("failed to execute set_file command on webview");
}

pub fn set_project_path(webview: &mut WebView<()>, path: String, dir_structure: Vec<DiskEntry>) {
    let cmd = format!(
        r#"document.dispatchEvent(
            new CustomEvent("setprojectpath", {{ detail: {{ path: {}, dir_structure: {} }} }})
        );"#,
        Json::String(path),
        serde_json::to_string(&dir_structure).expect("unable to format DiskEntry to JSON string"),
    );
    webview.eval(&cmd).expect("failed to execute set_project_path command on webview");
}
