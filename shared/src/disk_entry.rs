use serde::Deserialize;
use yew::{ html, Html };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskEntry {
    path: String,
    is_dir: bool,
}

impl DiskEntry {
    pub fn new(path: String, is_dir: bool) -> Self {
        DiskEntry {
            path: path,
            is_dir: is_dir,
        }
    }

    pub fn render(&self) -> Html {
        if self.is_dir {
            html! {
                <li> {"DIR: "} {self.path.clone()} </li>
            }
        } else {
            html! {
                <li> {self.path.clone()} </li>
            }
        }
    }
}
