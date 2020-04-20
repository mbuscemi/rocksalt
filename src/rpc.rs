use rocksalt_shared::event::{ Detail, Event };
use serde::Serialize;
use web_view::WebView;

pub fn dispatch<D>(webview: &mut WebView<()>, detail: D)
    where D: Detail + Serialize
{
    webview
        .eval(&Event::command_for_webview(detail))
        .expect(format!("failed to execute {} command on webview", D::NAME).as_str());
}
