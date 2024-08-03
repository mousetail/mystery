use std::{
    ffi::OsStr,
    fs::{read_dir, DirEntry, OpenOptions, ReadDir},
    io::{Error, Read, Write},
    iter::{FilterMap, Flatten, Once},
    path::PathBuf,
};

fn search_filter(d: Result<DirEntry, Error>) -> Option<Box<dyn Iterator<Item = PathBuf>>> {
    let d = d.ok()?;
    let metadata = d.metadata().ok()?;

    if metadata.is_file() {
        Some(Box::new(std::iter::once(d.path())))
    } else if metadata.is_dir() {
        Some(Box::new(recursively_search(d.path())))
    } else {
        None
    }
}

fn recursively_search(directory: PathBuf) -> impl Iterator<Item = PathBuf> {
    read_dir(directory)
        .unwrap()
        .filter_map(search_filter)
        .flatten()
}

fn main() {
    for mystery_folder in read_dir("mysteries").unwrap().filter_map(|k| {
        let v = k.ok()?;
        v.metadata().ok()?.is_dir().then(|| v)
    }) {
        for file in recursively_search(mystery_folder.path()) {
            if file.extension() != Some(OsStr::new("md")) {
                continue;
            }

            println!("{file:?}");
            let mut text = String::new();
            OpenOptions::new()
                .read(true)
                .write(false)
                .open(&file)
                .expect("Failed to open file")
                .read_to_string(&mut text)
                .expect("Failed to read file content");

            let destination = PathBuf::from("frontend-build").join(file.strip_prefix("mysteries").unwrap());

            let html = markdown_extensions::render_markdown(&text);

            OpenOptions::new().write(true).create(true).open(destination).unwrap().write_all(html.as_bytes()).unwrap();
        }
    }
}
