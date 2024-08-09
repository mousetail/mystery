use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, DirEntry, OpenOptions},
    io::{Error, Read, Write},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

fn os_str_starts_with(string: &OsStr, prefix: &str) -> bool {
    string.as_bytes().get(..prefix.len()) == Some(prefix.as_bytes())
}

fn search_filter(d: Result<DirEntry, Error>) -> Option<Box<dyn Iterator<Item = PathBuf>>> {
    let d = d.ok()?;
    let metadata = d.metadata().ok()?;

    if os_str_starts_with(&d.file_name(), ".") {
        return None;
    };
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

struct Page {
    markdown_location: PathBuf,
}

enum NavItemKind {
    Page(Page),
    Folder(Vec<NavItem>),
}

struct NavItem {
    name: String,
    kind: NavItemKind,
}

fn get_or_insert<'a>(list: &'a mut Vec<NavItem>, name: &String) -> &'a mut Vec<NavItem> {
    if let Some(new_nav_item) = list.iter_mut().find(|child| child.name.as_str() == name)
    {
        return match &mut new_nav_item.kind {
            NavItemKind::Folder(k) => k,
            NavItemKind::Page(_) => panic!("Expected category, got page at {name:?}"),
        }
    }

    let new_nav_item = NavItem {
        name: name.to_owned(),
        kind: NavItemKind::Folder(vec![]),
    };
    list.push(new_nav_item);
    let new_nav_item = &mut list[list.len() - 1];

    match &mut new_nav_item.kind {
        NavItemKind::Folder(k) => return k,
        NavItemKind::Page(p) => unreachable!("Expected category, got page at {name:?}"),
    }
}

fn main() {
    let mut handlebars = handlebars::Handlebars::new();

    let base_template = PathBuf::from("mysteries/base.hbs");
    handlebars
        .register_template_file("base", base_template)
        .unwrap();

    fn insert_page(root: &mut Vec<NavItem>, item: Page, path: Vec<String>) {
        let mut node = root;

        let [path @ .., page_name] = path.as_slice() else {
            panic!("Expected non-empty path");
        };

        for folder in path {
            let node = get_or_insert(node, folder);
        }

        node.push(NavItem {
            name: page_name.clone(),
            kind: NavItemKind::Page(item),
        })
    }

    for mystery_folder in read_dir("mysteries").unwrap().filter_map(|k| {
        let v = k.ok()?;
        v.metadata().ok()?.is_dir().then(|| v)
    }) {
        let mut nav_item_root = vec![];

        let template = mystery_folder.path().join("./template.hbs");
        let template_name = mystery_folder.file_name().to_str().unwrap().to_string();
        handlebars
            .register_template_file(&template_name, template)
            .unwrap();

        for file in recursively_search(mystery_folder.path()) {
            if file.extension() != Some(OsStr::new("md")) {
                continue;
            }

            let relative_path = file
                .strip_prefix("mysteries")
                .unwrap()
                .components()
                .map(|d| d.as_os_str().to_str().unwrap().to_owned())
                .collect();

            insert_page(
                &mut nav_item_root,
                Page {
                    markdown_location: file,
                },
                relative_path,
            );
        }

        // let mut text = String::new();
        // OpenOptions::new()
        //     .read(true)
        //     .write(false)
        //     .open(&file)
        //     .expect("Failed to open file")
        //     .read_to_string(&mut text)
        //     .expect("Failed to read file content");

        // let mut destination = PathBuf::from("frontend-build").join(relative_path);
        // destination.set_extension("html");

        // let html = markdown_extensions::render_markdown(&text);

        // let output_file = OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .truncate(true)
        //     .open(destination)
        //     .unwrap();

        // handlebars
        //     .render_to_write(
        //         &template_name,
        //         &HashMap::from([
        //             ("parent", "base"),
        //             ("html", &html),
        //             ("grey", "bob"),
        //             ("name", file.file_name().unwrap().to_str().unwrap()),
        //         ]),
        //         output_file,
        //     )
        //     .unwrap();
    }
}
