#![recursion_limit="256"]

#[macro_use]
extern crate stdweb;

mod event;
mod file;
mod message;
pub mod model;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

use event::{
    set_file::SetFileEvent,
    set_project_path::SetProjectPathEvent,
    JsRegistration
};
use file::File;
use message::Message;
use model::Model;

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut set_file_event = SetFileEvent::default();
        set_file_event.setup(&link);

        let mut set_project_path_event = SetProjectPathEvent::default();
        set_project_path_event.setup(&link);

        Model {
            link: link,
            project_path: None,
            file: File::empty(),
        }
    }

    fn destroy(&mut self) {
        //TODO: save off events and destroy them
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
                        match self.project_path {
                            None => html! {
                                <button id="open-project-folder-button" onclick=self.link.callback(|_| Message::OpenProject)>
                                    { "Open Project Folder" }
                                </button>
                            },
                            _ => html! {
                                <button id="close-button" onclick=self.link.callback(|_| Message::CloseProject)>
                                    { "✖" }
                                </button>
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
