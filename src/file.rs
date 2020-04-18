use rocksalt_shared::disk_entry::DiskEntry;
use std::io::prelude::*;
use std::fs::{ File, metadata };
use ignore::Walk;

pub fn read(path: String) -> String {
     let mut file = File::open(path).expect("unable to open file");
     let mut buffer = String::new();
     file.read_to_string(&mut buffer).expect("unable to read the file");
     serde_json::to_string(&buffer).expect("unable to JSON encode file contents").to_string()
}

pub fn dir_structure(path: String) -> Vec<DiskEntry> {
    let mut disk_entries: Vec<DiskEntry> = Vec::new();

    for result in Walk::new(path) {
        match result {
            Ok(entry) => {
                let display_value = entry.path().display();
                disk_entries.push(
                    DiskEntry::new(display_value.to_string(), is_dir(display_value.to_string()))
                );
            },
            Err(_) => (),
        }
    }

    disk_entries
}

fn is_dir(file_name:String) -> bool {
    let md = metadata(file_name.to_string()).unwrap();
    return md.is_dir();
}
