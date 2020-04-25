use serde::{ Serialize, Deserialize };

use crate::file_system::path::Path;
use crate::file_system::file::{ File, Named };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlainText {
    path: Path,
    raw: String
}

impl Named for PlainText {
    const NAME: &'static str = "plaintext";
}

impl File for PlainText {
    fn name(&self) -> String {
        self.path.filename.clone()
    }
    
    fn text(&self) -> String {
        self.raw.clone()
    }
}

impl PlainText {
    pub fn parse(path: &String, raw: &String) -> Self {
        PlainText {
            path: Path::create(path),
            raw: raw.to_string()
        }
    }
}