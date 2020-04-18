#![recursion_limit="256"]

#[macro_use]
extern crate stdweb;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

mod event;
mod file;
mod message;
pub mod model;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use event::{
    Event,
    set_file::SetFile,
    set_project_path::SetProjectPath,
};
use file::File;
use message::Message;
use model::Model;

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let events: [Event; 2] = [
            Event::new::<SetFile>(&link, String::from("setfile")),
            Event::new::<SetProjectPath>(&link, String::from("setprojectpath")),
        ];

        Model {
            link: link,
            events: events,
            project_path: None,
            file: File::empty(),
        }
    }

    fn destroy(&mut self) {
        for event in self.events.iter() {
            event.destroy();
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::OpenFile => {
                js! { external.invoke(JSON.stringify({ msg: "OpenFile" })); }
            },
            Message::SetFile(contents) => {
                self.file = File::new(contents);
            },
            Message::OpenProject => {
                js! { external.invoke(JSON.stringify({ msg: "OpenProject" })); }
            },
            Message::SetProjectPath(path) => {
                self.project_path = Some(path);
            },
            Message::CloseProject => {
                self.project_path = None;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <header>
                    // TODO: support opening a file through the project explorer
                    // <button onclick=self.link.callback(|_| Message::OpenFile)>{ "Open File" }</button>
                    {
                        match &self.project_path {
                            None => html! {
                                <button id="open-project-folder-button" onclick=self.link.callback(|_| Message::OpenProject)>
                                    { "Open Project Folder" }
                                </button>
                            },
                            Some(path) => html! {
                                <div>
                                    <span id="project-path">
                                        <strong>{ "Project Path: " }</strong> {path}
                                    </span>
                                    <button id="close-button" onclick=self.link.callback(|_| Message::CloseProject)>
                                        { "✖" }
                                    </button>
                                </div>
                            },
                        }
                    }
                </header>
                <section id="main-editor">
                    <div id="editor">{ &self.file.contents }</div>
                </section>
                <footer></footer>
            </div>
        }
    }
}
