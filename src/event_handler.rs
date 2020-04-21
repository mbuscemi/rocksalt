extern crate tinyfiledialogs as tfd;

use web_view::{ WebView, WVResult };
use rocksalt_shared::event::{
    set_file::SetFile,
    set_project_path::SetProjectPath
};
use rocksalt_shared::file_system::disk_entry::DiskEntry;
use rocksalt_shared::message::WebviewMessage;
use rocksalt_shared::utils::{ on_ok, on_some };

use crate::file;
use crate::rpc;

pub fn handle(webview: &mut WebView<()>, arg: &str) -> WVResult {
    on_ok(serde_json::from_str(arg), |message| {
        match message {
            WebviewMessage::SelectFile => {
                println!("SelectFile invoked");
                on_some(
                    tfd::open_file_dialog("Open File", "", None),
                    |path| {
                        rpc::dispatch(webview, SetFile {
                            contents: file::read(&path)
                        });
                    }
                );
            },

            WebviewMessage::OpenFile => {
                println!("OpenFile invoked");
            }

            WebviewMessage::SelectProject => {
                println!("SelectProject invoked");
                on_some(
                    tfd::select_folder_dialog("Open Project Folder", ""),
                    |path| {
                        let dir_structure: Vec<DiskEntry> = file::dir_structure(&path);
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
