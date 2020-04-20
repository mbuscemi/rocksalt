extern crate tinyfiledialogs as tfd;

use web_view::{ WebView, WVResult };
use rocksalt_shared::event::{
    set_file::SetFile,
    set_project_path::SetProjectPath
};
use rocksalt_shared::file_system::disk_entry::DiskEntry;

use crate::file;
use crate::message::Message;
use crate::rpc;

pub fn handle(webview: &mut WebView<()>, arg: &str) -> WVResult {
    match serde_json::from_str(arg) {
        Ok(message) => {
            match message {
                Message::OpenFile => {
                    match tfd::open_file_dialog("Open File", "", None) {
                        Some(path) => {
                            rpc::dispatch(webview, SetFile{ contents: file::read(&path) });
                            Ok(())
                        },
                        None => Ok(())
                    }
                },

                Message::OpenProject => {
                    match tfd::select_folder_dialog("Open Project Folder", "") {
                        Some(path) => {
                            let dir_structure: Vec<DiskEntry> = file::dir_structure(&path);
                            rpc::dispatch(webview, SetProjectPath{
                                path: path,
                                dir_structure: dir_structure
                            });
                            Ok(())
                        },
                        None => Ok(())
                    }
                }
            }
        }
        Err(error) => {
            println!("Could not match event from webview: {}", error);
            Ok(())
        }
    }
}
