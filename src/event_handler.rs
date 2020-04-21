extern crate tinyfiledialogs as tfd;

use web_view::{ WebView, WVResult };
use rocksalt_shared::event::{
    set_file::SetFile,
    set_project_path::SetProjectPath
};
use rocksalt_shared::file_system;
use rocksalt_shared::file_system::disk_entry::DiskEntry;
use rocksalt_shared::message::WebviewMessage;
use rocksalt_shared::utils::{ on_ok, on_some };

use crate::rpc;

pub fn handle(webview: &mut WebView<()>, arg: &str) -> WVResult {
    on_ok(serde_json::from_str(arg), |message| {
        match message {
            WebviewMessage::SelectFile => {
                on_some(
                    tfd::open_file_dialog("Open File", "", None),
                    |path| {
                        rpc::dispatch(webview, SetFile {
                            contents: file_system::read_file(&path)
                        });
                    }
                );
            },

            WebviewMessage::OpenFile { path } => {
                rpc::dispatch(webview, SetFile {
                    contents: file_system::read_file(&path)
                });
            }

            WebviewMessage::SelectProject => {
                on_some(
                    tfd::select_folder_dialog("Open Project Folder", ""),
                    |path| {
                        let dir_structure: Vec<DiskEntry> = file_system::dir_structure(&path);
                        rpc::dispatch(webview, SetProjectPath {
                            path: path,
                            dir_structure: dir_structure
                        });
                    }
                );
            },
        };
    });

    Ok(())
}
