#![recursion_limit="256"]

#[macro_use]
extern crate stdweb;

mod file;
mod message;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::Value;
use file::File;
use message::Message;

pub struct Model {
    link: ComponentLink<Self>,
    project_path: Option<String>,
    file: File,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let set_file_callback = link.callback(|content: String| Message::SetFile(content));

        let js_set_file_callback = move |value: Value| {
            set_file_callback.emit(
                value
                    .into_string()
                    .expect("unable to parse payload from setfile")
            )
        };

        js! {
            var set_file_callback = @{js_set_file_callback};
            document.addEventListener("setfile", event => set_file_callback(event.detail.contents));
        };

        let set_project_path_callback = link.callback(|path: String| Message::SetProjectPath(path));

        let js_set_project_path_callback = move |value: Value| {
            set_project_path_callback.emit(
                value
                    .into_string()
                    .expect("unable to parse payload from setprojectpath")
            )
        };

        js! {
            var set_project_path_callback = @{js_set_project_path_callback};
            document.addEventListener("setprojectpath", event => set_project_path_callback(event.detail.path));
        };

        Model {
            link: link,
            project_path: None,
            file: File::empty(),
        }
    }

    fn destroy(&mut self) {
        js! {
            set_file_callback.drop();
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
