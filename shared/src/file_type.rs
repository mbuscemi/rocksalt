use serde::Deserialize;
use std::fs::metadata;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FileType {
    Directory,
    Markdown,
    Liquid,
    Html,
    Css,
    Json,
    Yaml,
    Image,
    Other,
}

impl FileType {
    pub fn from(file_path: &String, filename: &String) -> Self {
        if is_dir(file_path) {
            Self::Directory
        } else {
            let mut parts: Vec<&str> = filename.split_terminator('.').collect();
            let extension: &str = parts.pop().unwrap();

            match extension {
                "md" => Self::Markdown,
                "liquid" => Self::Liquid,
                "html" => Self::Html,
                "css" => Self::Css,
                "json" => Self::Json,
                "yml" => Self::Yaml,
                "jpg" => Self::Image,
                "png" => Self::Image,
                "gif" => Self::Image,
                _ => Self::Other
            }
        }
    }

    pub fn to_css_class(&self) -> &str {
        match self {
            Self::Directory => "dir",
            Self::Markdown => "md",
            Self::Image => "img",
            _ => "file",
        }
    }
}

fn is_dir(file_path: &String) -> bool {
    let md = metadata(file_path.to_string()).unwrap();
    return md.is_dir();
}
