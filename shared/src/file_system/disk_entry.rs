use serde::{ Serialize, Deserialize };

use super::file_type::FileType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    pub full_path: String,
    pub filename: String,
    file_type: FileType,
    pub path_in_project: String,
    pub active: bool,
}

impl DiskEntry {
    pub fn new(file_path: String, project_path: String) -> Self {
        let filename = filename(&file_path);

        DiskEntry {
            full_path: file_path.clone(),
            filename: filename.clone(),
            file_type: FileType::from(&file_path, &filename),
            path_in_project: path_in_project(&file_path, &project_path),
            active: initial_active(&file_path, &project_path),
        }
    }

    pub fn toggled(&self) -> Self {
        DiskEntry {
            full_path: self.full_path.clone(),
            filename: self.filename.clone(),
            file_type: self.file_type.clone(),
            path_in_project: self.path_in_project.clone(),
            active: !self.active,
        }
    }

    pub fn project_path_sans_filename(&self) -> String {
        self.path_in_project.replace(&self.filename, "")
    }

    pub fn project_path_with_sep(&self) -> String {
        format!("{}{}", self.path_in_project, std::path::MAIN_SEPARATOR)
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }

    pub fn matches(&self, full_path: &String) -> bool {
        &self.full_path == full_path
    }

    pub fn css_class(&self) -> String {
        format!("{} {} {}",
            self.file_type.to_css_class(),
            if self.active { "active" } else { "" },
            if self.file_type.opennable_for_edit() { "opennable" } else { "no-edit" }
        )
    }
}

fn initial_active(full_path: &String, project_path: &String) -> bool {
    full_path == project_path
}

fn filename(file_path: &String) -> String {
    let reversed_filename: String = file_path.chars().rev().take_while(|&c| c != std::path::MAIN_SEPARATOR).collect();
    reversed_filename.chars().rev().collect()
}

fn path_in_project(file_path: &String, project_path: &String) -> String {
    file_path.replace(project_path, "")
}
