use serde::{ Serialize, Deserialize };

use super::file_type::FileType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Path {
    pub full: String,
    pub filename: String,
    pub file_type: FileType,
}

impl Path {
    pub fn create(file_path: &String) -> Self {
        let filename: String = filename(&file_path);

        Path {
            full: file_path.clone(),
            filename: filename.clone(),
            file_type: FileType::from(&file_path, &filename),
        }
    }
}

fn filename(file_path: &String) -> String {
    let reversed_filename: String = file_path.chars().rev().take_while(|&c| c != std::path::MAIN_SEPARATOR).collect();
    reversed_filename.chars().rev().collect()
}
