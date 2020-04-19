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

use rocksalt_shared::disk_entry::DiskEntry;
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
            Event::new::<SetFile>(&link),
            Event::new::<SetProjectPath>(&link),
        ];

        Model {
            link: link,
            events: events,
            project_path: None,
            project_structure: None,
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
            Message::SetProjectPath(path, disk_entries) => {
                self.project_path = Some(path);
                self.project_structure = Some(disk_entries);
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
                    <>
                        <h1 class="active">{ "Project" }</h1>
                        {project_hierarchy(&self.project_structure)}
                    </>
                }
            ),
            None => (
                html! {
                    <header>
                        // TODO: support opening a file through the project explorer
                        // <button onclick=self.link.callback(|_| Message::OpenFile)>{ "Open File" }</button>
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

fn project_hierarchy(project_structure: &Option<Vec<DiskEntry>>) -> Html {
    match project_structure {
        Some(structure) => {
            let mut mut_structure = structure.clone();
            let top_dir = mut_structure.remove(0);

            render_dir(top_dir, &mut mut_structure)
        },
        None => html! {},
    }
}

//TODO: figure out why deeply nested files aren't displaying
fn render_dir(top_dir: DiskEntry, rest: &mut Vec<DiskEntry>) -> Html {
    let top_dir_project_path = format!("{}{}", top_dir.path_in_project, std::path::MAIN_SEPARATOR);

    let (this_dir_entries, other_entries): (Vec<DiskEntry>, Vec<DiskEntry>)
        = rest.drain(..).partition(|entry| entry.project_path_sans_filename() == top_dir_project_path );

    let (mut these_folders, mut these_files): (Vec<DiskEntry>, Vec<DiskEntry>)
        = this_dir_entries.into_iter().partition(|entry| entry.is_dir() );

    these_folders.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));
    these_files.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));

    html! {
        <li class="dir">
            { top_dir.filename }
            <ul>
                { these_folders.iter().map(|entry| render_dir(entry.clone(), &mut other_entries.clone())).collect::<Html>() }
                { these_files.iter().map(|entry| html! { <li class="file">{entry.filename.clone()}</li> }).collect::<Html>() }
            </ul>
        </li>
    }
}
