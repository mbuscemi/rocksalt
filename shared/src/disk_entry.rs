use serde::Deserialize;
use std::fs::metadata;
use yew::{ html, Html };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    full_path: String,
    path_in_project: String,
    is_dir: bool,
}

impl DiskEntry {
    pub fn new(file_path: String, project_path: String) -> Self {

        DiskEntry {
            full_path: file_path.clone(),
            path_in_project: file_path.clone().replace(&project_path, ""),
            is_dir: is_dir(file_path),
        }
    }

    pub fn render(&self) -> Html {
        if self.is_dir {
            html! {
                <li class="dir">{self.path_in_project.clone()}</li>
            }
        } else {
            html! {
                <li class="file">{self.path_in_project.clone()}</li>
            }
        }
    }
}

fn is_dir(file_name:String) -> bool {
    let md = metadata(file_name.to_string()).unwrap();
    return md.is_dir();
}
