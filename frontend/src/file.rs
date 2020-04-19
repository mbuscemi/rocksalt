pub struct File {
    pub contents: String,
}

impl File {
    pub fn new(contents: String) -> Self {
        File { contents: contents }
    }
}
