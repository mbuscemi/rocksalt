pub mod model;
mod view;
mod event;

use crate::event::Event;
use rocksalt_shared::event::{
    set_file::SetFile,
    set_project_path::SetProjectPath,
};
use rocksalt_shared::file_system::file::{ cobalt_markdown::CobaltMarkdown, plain_text::PlainText };
use rocksalt_shared::message::{ WebviewMessage, YewMessage };
use wasm_bindgen::prelude::*;
use yew::{html, Component, Context, Html};

use model::Model;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Model>::new().render();
}

impl Component for Model {
    type Message = YewMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let events = vec![
            Event::create_for_yew::<SetFile<PlainText>>(ctx.link().callback(|msg| msg)),
            Event::create_for_yew::<SetFile<CobaltMarkdown>>(ctx.link().callback(|msg| msg)),
            Event::create_for_yew::<SetProjectPath>(ctx.link().callback(|msg| msg)),
        ];

        Model {
            events,
            project_path: None,
            project_structure: None,
            file: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
            YewMessage::UnsetFile => {
                self.file = None;
            }
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="page">
                { self.header(ctx) }
                <section id="main-editor">
                    <div id="project-panel">
                        { self.project_contents(ctx) }
                    </div>
                    { self.editor(ctx) }
                </section>
                { self.footer() }
            </div>
        }
    }
}
