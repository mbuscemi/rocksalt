use serde::{ Serialize, Deserialize };

use super::file_type::FileType;
use crate::file_system::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    pub path: Path,
    pub project_path: String,
    pub active: bool,
}

impl DiskEntry {
    pub fn new(file_path: String, project_path: String) -> Self {
        DiskEntry {
            path: Path::create(&file_path),
            project_path: path_in_project(&file_path, &project_path),
            active: initial_active(&file_path, &project_path),
        }
    }

    pub fn toggled(&self) -> Self {
        DiskEntry {
            path: self.path.clone(),
            project_path: self.project_path.clone(),
            active: !self.active,
        }
    }

    pub fn project_path_sans_filename(&self) -> String {
        self.project_path.replace(&self.path.filename, "")
    }

    pub fn project_path_with_sep(&self) -> String {
        format!("{}{}", self.project_path, std::path::MAIN_SEPARATOR)
    }

    pub fn is_dir(&self) -> bool {
        self.path.file_type == FileType::Directory
    }

    pub fn matches(&self, full_path: &String) -> bool {
        &self.path.full == full_path
    }

    pub fn css_class(&self) -> String {
        format!("{} {} {}",
            self.path.file_type.to_css_class(),
            if self.active { "active" } else { "" },
            if self.path.file_type.opennable_for_edit() { "opennable" } else { "no-edit" }
        )
    }

    pub fn opennable_for_edit(&self) -> bool {
        self.path.file_type.opennable_for_edit()
    }
}

fn path_in_project(file_path: &String, project_path: &String) -> String {
    file_path.replace(project_path, "")
}

fn initial_active(full_path: &String, project_path: &String) -> bool {
    full_path == project_path
}
