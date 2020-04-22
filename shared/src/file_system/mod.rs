use std::io::prelude::*;
use std::fs::File;
use ignore::Walk;

pub mod disk_entry;
pub mod file;
pub mod file_type;

use disk_entry::DiskEntry;

pub fn read_file(path: &String) -> String {
     let mut file = File::open(path).expect("unable to open file");
     let mut buffer = String::new();
     file.read_to_string(&mut buffer).expect("unable to read the file");
     serde_json::to_string(&buffer).expect("unable to JSON encode file contents").to_string()
}

pub fn dir_structure(path: &String) -> Vec<DiskEntry> {
    let mut disk_entries: Vec<DiskEntry> = Vec::new();

    for result in Walk::new(path.clone()) {
        match result {
            Ok(entry) => {
                let display_value = entry.path().display();
                disk_entries.push(
                    DiskEntry::new(display_value.to_string(), path.clone())
                );
            },
            Err(_) => (),
        }
    }

    disk_entries
}
