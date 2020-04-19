use rocksalt_shared::disk_entry::DiskEntry;
use yew::{html, Html};

use crate::message::Message;
use crate::model::Model;

impl Model {
    pub fn header(&self) -> Html {
        match &self.project_path {
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
                }
            ),
        }
    }

    pub fn project_contents(&self) -> Html {
        match &self.project_path {
            Some(_) => (
                html! {
                    <>
                        <h1 class="active">{ "Project" }</h1>
                        {self.project_hierarchy()}
                    </>
                }
            ),
            None => (
                html! {
                    <h1 class="inactive">{ "Project" }</h1>
                }
            ),
        }
    }

    pub fn project_hierarchy(&self) -> Html {
        match &self.project_structure {
            Some(structure) => {
                let mut mut_structure = structure.clone();
                let top_dir = mut_structure.remove(0);

                html! {
                    <ul>
                        { self.render_dir(&top_dir, &mut mut_structure) }
                    </ul>
                }
            },
            None => html! {},
        }
    }

    //TODO: figure out why deeply nested files aren't displaying
    pub fn render_dir(&self, top_dir: &DiskEntry, rest: &mut Vec<DiskEntry>) -> Html {
        let top_dir_clone = top_dir.clone();

        let (this_dir_entries, other_entries): (Vec<DiskEntry>, Vec<DiskEntry>)
            = rest.drain(..).partition(|entry| entry.project_path_sans_filename() == top_dir.project_path_with_sep());

        let (mut these_folders, mut these_files): (Vec<DiskEntry>, Vec<DiskEntry>)
            = this_dir_entries.into_iter().partition(|entry| entry.is_dir());

        these_folders.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));
        these_files.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));

        html! {
            <li
                class={top_dir.css_class()}
                onclick=self.link.callback(move |_| Message::ToggleHierarchy(top_dir_clone.full_path.clone()))
            >
                <span>{ top_dir.filename.clone() }</span>
                <ul>
                    { these_folders.iter().map(|entry| self.render_dir(entry, &mut other_entries.clone())).collect::<Html>() }
                    { these_files.iter().map(|entry| self.render_file(entry)).collect::<Html>() }
                </ul>
            </li>
        }
    }

    pub fn render_file(&self, entry: &DiskEntry) -> Html {
        html! {
            <li class={entry.css_class()} onclick=self.link.callback(|_| Message::Noop)>
                <span>{entry.filename.clone()}</span>
            </li>
        }
    }
}
