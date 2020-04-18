use yew::ComponentLink;

use crate::event::Event;
use crate::file::File;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub events: [Event; 2],
    pub project_path: Option<String>,
    pub file: File,
}
