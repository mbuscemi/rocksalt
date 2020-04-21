#![recursion_limit="512"]

mod file;
pub mod model;
mod view;

use rocksalt_shared::event::{
    Event,
    set_file::SetFile,
    set_project_path::SetProjectPath,
};
use rocksalt_shared::message::{ WebviewMessage, YewMessage };
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use file::File;
use model::Model;

impl Component for Model {
    type Message = YewMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let events: [Event; 2] = [
            Event::create_for_yew::<Model, SetFile>(&link),
            Event::create_for_yew::<Model, SetProjectPath>(&link),
        ];

        Model {
            link: link,
            events: events,
            project_path: None,
            project_structure: None,
            file: None,
        }
    }

    fn destroy(&mut self) {
        for event in self.events.iter() {
            event.destroy_for_yew();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            YewMessage::SelectFile => {
                Event::invoke_on_webview(WebviewMessage::SelectFile);
            },
            YewMessage::OpenFile(_path) => {
                Event::invoke_on_webview(WebviewMessage::OpenFile);
            },
            YewMessage::SetFile(contents) => {
                self.file = Some(File::new(contents));
            },
            YewMessage::OpenProject => {
                Event::invoke_on_webview(WebviewMessage::SelectProject);
            },
            YewMessage::SetProjectPath(path, disk_entries) => {
                self.project_path = Some(path);
                self.project_structure = Some(disk_entries);
            },
            YewMessage::CloseProject => {
                self.project_path = None;
            },
            YewMessage::ToggleHierarchy(full_path) => {
                self.toggle_entry_at(&full_path);
            },
            YewMessage::Noop => {},
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="page">
                { self.header() }
                <section id="main-editor">
                    <div id="project-panel">
                        { self.project_contents() }
                    </div>
                    { self.editor() }
                </section>
                { self.footer() }
            </div>
        }
    }
}
