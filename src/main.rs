use std::{fs, io};

mod lib;

fn main() {
    println!("Plexify renames files to be applicable for plex indexing");
    let mut entries = fs::read_dir(".").unwrap();
        // .map(|res| res.map(|e| e.unwrap().path()))
        // .collect::<Result<Vec<_>, io::Error>>();
    for file in entries {
        println!("Name: {}", file.unwrap().path().display())
    }
}
