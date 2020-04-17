pub struct File {
    pub contents: String,
}

impl File {
    pub fn new(contents: String) -> Self {
        File { contents: contents }
    }

    pub fn empty() -> Self {
        File { contents: String::new() }
    }
}
