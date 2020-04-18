use serde::Deserialize;
use yew::{ html, Html };

//TODO: Extract this into a module that can be shared by both the Yew and WebView layers

#[derive(Deserialize, Debug, Clone)]
pub struct DiskEntry {
    path: String,
    is_dir: bool,
}

impl DiskEntry {
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
