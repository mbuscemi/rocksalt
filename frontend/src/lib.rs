#![recursion_limit="512"]

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
        let (header, project_contents) = match &self.project_path {
            Some(path) => (
                html! {
                    <header>
                        <span id="project-path">
                            {path}
                        </span>
                        <button id="close-button" onclick=self.link.callback(|_| Message::CloseProject)>
                            { "✖" }
                        </button>
                    </header>
                },
                html! {
                    <h1 class="active">{ "Project" }</h1>
                }
            ),
            None => (
                html! {
                    <header>
                        <button id="open-project-folder-button" onclick=self.link.callback(|_| Message::OpenProject)>
                            { "Open Project Folder" }
                        </button>
                    </header>
                },
                html! {
                    <h1 class="inactive">{ "Project" }</h1>
                }
            ),
        };

        html! {
            <div id="page">
                // TODO: support opening a file through the project explorer
                // <button onclick=self.link.callback(|_| Message::OpenFile)>{ "Open File" }</button>
                {header}
                <section id="main-editor">
                    <div id="project-panel">
                        {project_contents}
                    </div>
                    <div id="editor">
                        { &self.file.contents }
                    </div>
                </section>
                <footer></footer>
            </div>
        }
    }
}
