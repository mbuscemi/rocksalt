use rocksalt_shared::file_system::disk_entry::DiskEntry;
use yew::ComponentLink;

use crate::event::Event;
use crate::file::File;

pub struct Model {
    pub link: ComponentLink<Self>,
    pub events: [Event; 2],
    pub project_path: Option<String>,
    pub project_structure: Option<Vec<DiskEntry>>,
    pub file: Option<File>,
}

impl Model {
    pub fn toggle_entry_at(&mut self, full_path: &String) {
        self.project_structure =
            self.project_structure.as_ref().map(|structure| {
                structure.iter().map(|entry| {
                    if entry.matches(&full_path) {
                        entry.toggled()
                    } else {
                        entry.clone()
                    }
                }).collect()
            });
    }
}
