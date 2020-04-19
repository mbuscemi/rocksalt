use serde::Deserialize;
use std::fs::metadata;
use yew::{ html, Html };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    full_path: String,
    filename: String,
    path_in_project: String,
    is_dir: bool,
}

impl DiskEntry {
    pub fn new(file_path: String, project_path: String) -> Self {
        DiskEntry {
            full_path: file_path.clone(),
            filename: filename(&file_path),
            path_in_project: path_in_project(&file_path, &project_path),
            is_dir: is_dir(file_path),
        }
    }

    pub fn render(&self) -> Html {
        html! {
            <li class={if self.is_dir { "dir" } else { "file" }}>{self.filename.clone()}</li>
        }
    }
}

fn filename(file_path: &String) -> String {
    let reversed_filename: String = file_path.chars().rev().take_while(|&c| c != std::path::MAIN_SEPARATOR).collect();
    reversed_filename.chars().rev().collect()
}

fn path_in_project(file_path: &String, project_path: &String) -> String {
    file_path.replace(project_path, "")
}

fn is_dir(file_path: String) -> bool {
    let md = metadata(file_path.to_string()).unwrap();
    return md.is_dir();
}
