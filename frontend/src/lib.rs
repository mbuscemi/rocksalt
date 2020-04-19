#![recursion_limit="512"]

#[macro_use]
extern crate stdweb;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

mod file;
pub mod model;
mod view;

use rocksalt_shared::event::{
    Event,
    set_file::SetFile,
    set_project_path::SetProjectPath,
};
use rocksalt_shared::event::message::Message;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use file::File;
use model::Model;

impl Component for Model {
    type Message = Message;
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
            Message::OpenFile => {
                js! { external.invoke(JSON.stringify({ msg: "OpenFile" })); }
            },
            Message::SetFile(contents) => {
                self.file = Some(File::new(contents));
            },
            Message::OpenProject => {
                js! { external.invoke(JSON.stringify({ msg: "OpenProject" })); }
            },
            Message::SetProjectPath(path, disk_entries) => {
                self.project_path = Some(path);
                self.project_structure = Some(disk_entries);
            },
            Message::CloseProject => {
                self.project_path = None;
            },
            Message::ToggleHierarchy(full_path) => {
                self.toggle_entry_at(&full_path);
            },
            Message::Noop => {},
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
