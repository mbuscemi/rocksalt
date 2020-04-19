use serde::Deserialize;

use crate::file_type::FileType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    full_path: String,
    pub filename: String,
    file_type: FileType,
    pub path_in_project: String,
}

impl DiskEntry {
    pub fn new(file_path: String, project_path: String) -> Self {
        let filename = filename(&file_path);

        DiskEntry {
            full_path: file_path.clone(),
            filename: filename.clone(),
            file_type: FileType::from(&file_path, &filename),
            path_in_project: path_in_project(&file_path, &project_path),
        }
    }

    pub fn project_path_sans_filename(&self) -> String {
        self.path_in_project.replace(&self.filename, "")
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }
}

fn filename(file_path: &String) -> String {
    let reversed_filename: String = file_path.chars().rev().take_while(|&c| c != std::path::MAIN_SEPARATOR).collect();
    reversed_filename.chars().rev().collect()
}

fn path_in_project(file_path: &String, project_path: &String) -> String {
    file_path.replace(project_path, "")
}
