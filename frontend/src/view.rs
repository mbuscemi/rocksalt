use rocksalt_shared::file_system::disk_entry::DiskEntry;
use rocksalt_shared::message::YewMessage;
use web_sys::MouseEvent;
use yew::{html, Context, Html};

use crate::model::Model;

impl Model {
    pub fn header(&self, ctx: &Context<Model>) -> Html {
        match &self.project_path {
            Some(_) => html! {
                <header>
                    <div id="header-left">
                        { self.save_button() }
                    </div>
                    <div id="header-right">
                        <button id="close-button" onclick={ctx.link().callback(|_| YewMessage::CloseProject)}>
                            { "✖" }
                        </button>
                    </div>
                </header>
            },
            None => html! {
                <header>
                    <button id="open-project-folder-button" onclick={ctx.link().callback(|_| YewMessage::OpenProject)}>
                        { "Open Project Folder" }
                    </button>
                </header>
            },
        }
    }

    pub fn save_button(&self) -> Html {
        match self.file {
            Some(_) => html! { <button id="save-button">{ "Save" }</button> },
            None => html! { <button id="save-button" disabled={true}>{ "Save" }</button> },
        }
    }

    pub fn footer(&self) -> Html {
        match &self.project_path {
            Some(path) => html! {
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
            },
            None => html! {
                <footer>
                </footer>
            },
        }
    }

    pub fn project_contents(&self, ctx: &Context<Model>) -> Html {
        match &self.project_path {
            Some(_) => html! {
                <>
                    <h1 class="active">{ "Project" }</h1>
                    {self.project_hierarchy(ctx)}
                </>
            },
            None => html! {
                <h1 class="inactive">{ "Project" }</h1>
            },
        }
    }

    pub fn project_hierarchy(&self, ctx: &Context<Model>) -> Html {
        match &self.project_structure {
            Some(structure) => {
                let mut mut_structure = structure.clone();
                let top_dir = mut_structure.remove(0);

                html! {
                    <ul>
                        { self.render_dir(ctx, &top_dir, &mut mut_structure) }
                    </ul>
                }
            },
            None => html! {},
        }
    }

    pub fn render_dir(&self, ctx: &Context<Model>, top_dir: &DiskEntry, rest: &mut Vec<DiskEntry>) -> Html {
        let top_dir_clone = top_dir.clone();

        let (this_dir_entries, other_entries): (Vec<DiskEntry>, Vec<DiskEntry>)
            = rest.drain(..).partition(|entry| entry.project_path_sans_filename() == top_dir.project_path_with_sep());

        let (mut these_folders, mut these_files): (Vec<DiskEntry>, Vec<DiskEntry>)
            = this_dir_entries.into_iter().partition(|entry| entry.is_dir());

        these_folders.sort_by(|a, b| a.path.filename.to_lowercase().cmp(&b.path.filename.to_lowercase()));
        these_files.sort_by(|a, b| a.path.filename.to_lowercase().cmp(&b.path.filename.to_lowercase()));

        html! {
            <li
                class={top_dir.css_class()}
                onclick={ctx.link().callback(move |e: MouseEvent| {
                    e.stop_propagation();
                    YewMessage::ToggleHierarchy(top_dir_clone.path.full.clone())
                })}
            >
                <span>{ top_dir.path.filename.clone() }</span>
                <ul>
                    { these_folders.iter().map(|entry| self.render_dir(ctx, entry, &mut other_entries.clone())).collect::<Html>() }
                    { these_files.iter().map(|entry| self.render_file(ctx, entry)).collect::<Html>() }
                </ul>
            </li>
        }
    }

    pub fn render_file(&self, ctx: &Context<Model>, entry: &DiskEntry) -> Html {
        if entry.opennable_for_edit() {
            let entry_clone = entry.clone();
            html! {
                <li class={entry.css_class()}
                    onclick={ctx.link().callback(move |e: MouseEvent| {
                        e.stop_propagation();
                        YewMessage::OpenFile {
                            path: entry_clone.path.full.clone(),
                            file_type: entry_clone.path.file_type.clone(),
                        }
                    })}
                >
                    <span>{entry.path.filename.clone()}</span>
                </li>
            }
        } else {
            html! {
                <li class={entry.css_class()} onclick={ctx.link().callback(|e: MouseEvent| {
                    e.stop_propagation();
                    YewMessage::Noop
                })}>
                    <span>{entry.path.filename.clone()}</span>
                </li>
            }
        }
    }

    pub fn editor(&self, ctx: &Context<Model>) -> Html {
        match &self.file {
            Some(file) => html! {
                <div id="editor">
                    <div id="editor-toolbar">
                        <div class="file-tab">
                            <div>{ file.name() }</div>
                            <button class="close" onclick={ctx.link().callback(|_| YewMessage::UnsetFile)}>{ "✖" }</button>
                        </div>
                    </div>
                    <div id="editor-panel">
                        <pre id="editor-pre">
                            <code id="editor-main" class="language-md" contenteditable="true">
                                { file.text() }
                            </code>
                        </pre>
                    </div>
                </div>
            },
            None => html! {
                <div id="editor">
                    <div id="editor-toolbar"></div>
                    <div id="editor-panel"></div>
                </div>
            },
        }
    }
}
