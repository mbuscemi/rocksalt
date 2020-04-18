pub mod set_file;
pub mod set_project_path;

use yew::ComponentLink;

use crate::model::Model;

pub trait JsRegistration {
    fn setup(&mut self, link: &ComponentLink<Model>);
    fn destroy(&self);
}
