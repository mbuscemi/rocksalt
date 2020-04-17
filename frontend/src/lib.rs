#![recursion_limit="256"]

#[macro_use]
extern crate stdweb;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use stdweb::Value;

struct File {
    contents: String,
}

impl File {
    fn new(contents: String) -> Self {
        File { contents: contents }
    }

    fn empty() -> Self {
        File { contents: String::new() }
    }
}

pub struct Model {
    link: ComponentLink<Self>,
    project_path: Option<String>,
    file: File,
}

pub enum Msg {
    OpenFile,
    SetFile(String),
    OpenProject,
    SetProjectPath(String),
    CloseProject,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let set_file_callback = link.callback(|content: String| Msg::SetFile(content));

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

        let set_project_path_callback = link.callback(|path: String| Msg::SetProjectPath(path));

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
            Msg::OpenFile => {
                js! { external.invoke(JSON.stringify({ msg: "OpenFile" })); }
            },
            Msg::SetFile(contents) => {
                self.file = File::new(contents);
            },
            Msg::OpenProject => {
                js! { external.invoke(JSON.stringify({ msg: "OpenProject" })); }
            },
            Msg::SetProjectPath(path) => {
                self.project_path = Some(path);
            },
            Msg::CloseProject => {
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
                                <button id="open-project-folder-button" onclick=self.link.callback(|_| Msg::OpenProject)>
                                    { "Open Project Folder" }
                                </button>
                            },
                            _ => html! {
                                <button id="close-button" onclick=self.link.callback(|_| Msg::CloseProject)>
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
