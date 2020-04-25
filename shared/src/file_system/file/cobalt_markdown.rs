use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

use crate::file_system::path::Path;
use crate::file_system::file::{ File, Named };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CobaltMarkdown {
    path: Path,
    title: Option<String>,
    description: Option<String>,
    layout: Option<String>,
    tags: Vec<String>,
    categories: Vec<String>,
    published_date: Option<String>,
    is_draft: bool,
    text: String,
}

impl Named for CobaltMarkdown {
    const NAME: &'static str = "cobaltmarkdown";
}

impl File for CobaltMarkdown {
    fn name(&self) -> String {
        self.path.filename.clone()
    }

    fn text(&self) -> String {
        self.text.clone()
    }
}

impl CobaltMarkdown {
    pub fn parse(path: &String, raw: &String) -> Self {
        let sections: Vec<&str> = raw.split("---").collect();
        let data: &str = sections.get(1).unwrap();
        let chunks: Vec<&str> = data.split("\\n").collect();
        let text: String = clean_text(sections.get(2).unwrap_or(&"").to_string());

        let props: HashMap<String, String> =
            chunks.iter().map(|elem| -> (String, String) {
                let parts: Vec<&str> = elem.splitn(2, ':').collect();
                let key: String = clean_data(parts.get(0).unwrap_or(&"").to_string());
                let value: String = clean_data(parts.get(1).unwrap_or(&"").to_string());
                (key, value)
            }).collect();

        let tags: Vec<String> = gather_metadata_list(String::from("tags"), data.to_string());
        let categories: Vec<String> = gather_metadata_list(String::from("categories"), data.to_string());

        //TODO: figure out how to get a proper datetime involve; this one is not serializable by Serde
        //published_date: Option<DateTime<Utc>>,
        //use chrono::{ DateTime, TimeZone, Utc };
        //Utc.datetime_from_str(props.get("published_date").unwrap_or(&String::from("")), "%Y-%m-%d %H:%M:%S %z").ok()

        CobaltMarkdown{
            path: Path::create(path),
            title: props.get("title").map(|s| s.clone()),
            description: props.get("description").map(|s| s.clone()),
            layout: props.get("layout").map(|s| s.clone()),
            tags: tags,
            categories: categories,
            published_date: props.get("published_date").map(|s| s.clone()),
            is_draft: parse_is_draft(props.get("is_draft")),
            text: text,
        }
    }
}

fn clean_data(str: String) -> String {
    str.replace("\\\"", "").trim().to_string()
}

fn clean_text(str: String) -> String {
    str.replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"")
}

fn parse_is_draft(prop: Option<&String>) -> bool {
    match prop {
        Some(bool_string) => if bool_string == "true" { true } else { false },
        None => true,
    }
}

fn gather_metadata_list(name: String, data: String) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    let identifier: String = format!("{}:", name);
    let data_split: Vec<&str> = data.split(&identifier).collect();
    let mut rest: String = data_split.get(1).unwrap_or(&"").to_string();

    while rest.starts_with("\\n  - ") {
        rest = rest.replacen("\\n  - ", "", 1);

        let next_newline = rest.find("\\n").unwrap_or(0);
        match rest.get(0..next_newline) {
            Some(tag) => tags.push(tag.to_string()),
            None => (),
        };

        rest = rest.get(next_newline..).unwrap_or(&"").to_string();
    }

    tags
}
