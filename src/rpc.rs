use rocksalt_shared::file_system::disk_entry::DiskEntry;
use web_view::WebView;

pub fn set_file(webview: &mut WebView<()>, contents: String) {
    let cmd = format!(
        r#"document.dispatchEvent(
            new CustomEvent("setfile", {{ detail: {{ contents: {} }} }})
        );"#,
        serde_json::to_string(&contents).expect("failed to JSON encode file contents"),
    );
    webview.eval(&cmd).expect("failed to execute set_file command on webview");
}

pub fn set_project_path(webview: &mut WebView<()>, path: String, dir_structure: Vec<DiskEntry>) {
    let cmd = format!(
        r#"document.dispatchEvent(
            new CustomEvent("setprojectpath", {{ detail: {{ path: {}, dir_structure: {} }} }})
        );"#,
        serde_json::to_string(&path).expect("failed to JSON encode path"),
        serde_json::to_string(&dir_structure).expect("failed to format DiskEntry to JSON string"),
    );
    webview.eval(&cmd).expect("failed to execute set_project_path command on webview");
}
