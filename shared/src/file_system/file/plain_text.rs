use serde::{ Serialize, Deserialize };

use crate::file_system::file::{ File, Named };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlainText {
    raw: String
}

impl Named for PlainText {
    const NAME: &'static str = "plaintext";
}

impl File for PlainText {
    fn text(&self) -> String {
        self.raw.clone()
    }
}

impl PlainText {
    pub fn parse(raw: &String) -> Self {
        PlainText {
            raw: raw.to_string()
        }
    }
}