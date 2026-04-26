extern crate tinyfiledialogs as tfd;

use rocksalt_shared::event::{
    set_file::SetFile,
    set_project_path::SetProjectPath
};
use rocksalt_shared::file_system;
use rocksalt_shared::file_system::{ disk_entry::DiskEntry, file_type::FileType };
use rocksalt_shared::file_system::file::{ cobalt_markdown::CobaltMarkdown, plain_text::PlainText };
use rocksalt_shared::message::WebviewMessage;

use crate::rpc;

pub fn handle(arg: &str) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();

    if let Ok(message) = serde_json::from_str::<WebviewMessage>(arg) {
        match message {
            WebviewMessage::SelectFile => {
                if let Some(path) = tfd::open_file_dialog("Open File", "", None) {
                    commands.push(rpc::dispatch(SetFile {
                        file: PlainText::parse(&path, &file_system::read_file(&path)),
                    }));
                }
            },

            WebviewMessage::OpenFile { path, file_type } => {
                let cmd = match file_type {
                    FileType::Markdown => rpc::dispatch(SetFile {
                        file: CobaltMarkdown::parse(&path, &file_system::read_file(&path)),
                    }),
                    _ => rpc::dispatch(SetFile {
                        file: PlainText::parse(&path, &file_system::read_file(&path)),
                    }),
                };
                commands.push(cmd);
            },

            WebviewMessage::SelectProject => {
                if let Some(path) = tfd::select_folder_dialog("Open Project Folder", "") {
                    let dir_structure: Vec<DiskEntry> = file_system::dir_structure(&path);
                    commands.push(rpc::dispatch(SetProjectPath {
                        path: path,
                        dir_structure: dir_structure,
                    }));
                }
            },
        }
    }

    commands
}
