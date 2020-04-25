#![recursion_limit="512"]

pub mod model;
mod view;

use rocksalt_shared::event::{
    Event,
    set_file::SetFile,
    set_project_path::SetProjectPath,
};
use rocksalt_shared::file_system::file::{ cobalt_markdown::CobaltMarkdown, plain_text::PlainText };
use rocksalt_shared::message::{ WebviewMessage, YewMessage };
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use model::Model;

impl Component for Model {
    type Message = YewMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let events: [Event; 3] = [
            Event::create_for_yew::<Model, SetFile<PlainText>>(&link),
            Event::create_for_yew::<Model, SetFile<CobaltMarkdown>>(&link),
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
            YewMessage::OpenFile{ path, file_type } => {
                Event::invoke_on_webview(WebviewMessage::OpenFile{ path, file_type });
            },
            YewMessage::SetFile(file) => {
                self.file = Some(file);
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
                self.project_structure = None;
                self.file = None;
            },
            YewMessage::ToggleHierarchy(full_path) => {
                self.toggle_entry_at(&full_path);
            },
            YewMessage::Noop => {},
        }
        true
    }
    
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
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
