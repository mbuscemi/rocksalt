use rocksalt_shared::file_system::disk_entry::DiskEntry;
use rocksalt_shared::message::Message;
use yew::{html, Html};

use crate::model::Model;

impl Model {
    pub fn header(&self) -> Html {
        match &self.project_path {
            Some(_) => (
                html! {
                    <header>
                        <div id="header-left">
                            { self.save_button() }
                        </div>
                        <div id="header-right">
                            <button id="close-button" onclick=self.link.callback(|_| Message::CloseProject)>
                                { "✖" }
                            </button>
                        </div>
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

    pub fn save_button(&self) -> Html {
        match self.file {
            Some(_) => html! { <button id="save-button">{ "Save" }</button> },
            None => html! { <button id="save-button" disabled=true>{ "Save" }</button> },
        }
    }

    pub fn footer(&self) -> Html {
        match &self.project_path {
            Some(path) => (
                html! {
                    <footer>
                        <div id="footer-left">
                            <span id="project-path">
                                {path}
                            </span>
                        </div>
                        <div id="footer-right">
                            <button id="save-button">{ "Build Project" }</button>
                        </div>
                    </footer>
                }
            ),
            None => (
                html! {
                    <footer>
                    </footer>
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
        if entry.opennable_for_edit() {
            let entry_clone = entry.clone();
            html! {
                <li class={entry.css_class()}
                    onclick=self.link.callback(|_| Message::Noop)
                    ondoubleclick=self.link.callback(move |_| Message::OpenFile(entry_clone.full_path.clone()))
                >
                    <span>{entry.filename.clone()}</span>
                </li>
            }
        } else {
            html! {
                <li class={entry.css_class()} onclick=self.link.callback(|_| Message::Noop)>
                    <span>{entry.filename.clone()}</span>
                </li>
            }
        }
    }

    pub fn editor(&self) -> Html {
        let contents: &str =
            match &self.file {
                Some(file) => file.contents.as_ref(),
                None => "",
            };

        html! {
            <div id="editor">
                { contents }
            </div>
        }
    }
}
