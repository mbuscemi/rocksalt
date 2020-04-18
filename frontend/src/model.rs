use yew::ComponentLink;

use crate::file::File;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub project_path: Option<String>,
    pub file: File,
}
