use web_view::WebView;

pub fn set_file(webview: &mut WebView<()>, contents: String) {
    let cmd = format!(
        r#"document.dispatchEvent(
            new CustomEvent("setfile", {{ detail: {{ contents: '{}' }} }})
        );"#,
        contents
    );
    webview.eval(&cmd).expect("failed to execute set_file command on webview");
}

pub fn set_project_path(webview: &mut WebView<()>, path: String) {
    let cmd = format!(
        r#"document.dispatchEvent(
            new CustomEvent("setprojectpath", {{ detail: {{ path: '{}' }} }})
        );"#,
        path
    );
    webview.eval(&cmd).expect("failed to execute set_project_path command on webview");
}
