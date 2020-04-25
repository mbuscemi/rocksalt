use crate::file_system::file::File;

#[derive(Debug)]
pub struct PlainText {
    raw: String
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