pub mod cobalt_markdown;
pub mod plain_text;

pub trait File {
    fn text(&self) -> String;
}

pub trait Named {
    const NAME: &'static str;
}