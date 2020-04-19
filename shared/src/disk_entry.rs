use serde::Deserialize;
use std::fs::metadata;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    pub full_path: String,
    pub filename: String,
    pub path_in_project: String,
    pub is_dir: bool,
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

    pub fn project_path_sans_filename(&self) -> String {
        self.path_in_project.replace(&self.filename, "")
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
