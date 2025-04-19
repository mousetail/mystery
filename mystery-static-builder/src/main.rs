mod nav;

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{DirEntry, OpenOptions, read_dir},
    io::{Error, Read},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

use nav::{NavItem, Page, get_or_insert};
use serde::Serialize;
use tera::Tera;

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

struct RenderingContext<'a> {
    tera: &'a Tera,
    nav: NavItem,
    template_name: &'a str,
}

#[derive(Serialize)]
struct RenderData<'a> {
    nav: &'a NavItem,
    html: &'a str,
    name: &'a str,
    parent: &'a str,
    page: &'a [&'a str],
}

fn process_ex(source_file: PathBuf, context: &RenderingContext) {
    let mut text = String::new();

    let dest_file = PathBuf::from("frontend-build")
        .join(source_file.strip_prefix("mysteries").unwrap())
        .with_extension("html");

    println!("{source_file:?} {dest_file:?}");
    OpenOptions::new()
        .read(true)
        .write(false)
        .open(&source_file)
        .expect("Failed to open file")
        .read_to_string(&mut text)
        .expect("Failed to read file content");

    let html = markdown_extensions::render_markdown(&text);

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dest_file)
        .unwrap();

    let tera_context = tera::Context::from_serialize(RenderData {
        parent: "base",
        html: &html,
        nav: &context.nav,
        name: &source_file.file_name().unwrap().to_str().unwrap(),
        page: &["greg"],
    })
    .unwrap();

    context
        .tera
        .render_to(&context.template_name, &tera_context, output_file)
        .unwrap();
}

fn recursively_generate_pages(f: NavItem, context: &RenderingContext) {
    match f.kind {
        nav::NavItemKind::Page(page) => {
            process_ex(page.markdown_location, context);
        }
        nav::NavItemKind::Folder(hash_map) => {
            for (_name, value) in hash_map {
                // let mut source = source.clone();
                // let mut destination = destination.clone();
                // source.push(name.clone());
                // destination.push(name);

                recursively_generate_pages(value, context);
            }
        }
    }
}

fn main() {
    let mut tera = Tera::new("mysteries/**/*.html.jinja").unwrap();
    tera.autoescape_on(vec![".html.jinja"]);

    fn insert_page(root: &mut NavItem, item: Page, path: Vec<String>) {
        let mut node = root;

        let [path @ .., page_name] = path.as_slice() else {
            panic!("Expected non-empty path");
        };

        for folder in path {
            node = get_or_insert(
                node,
                NavItem {
                    name: folder.clone(),
                    kind: nav::NavItemKind::Folder(HashMap::new()),
                },
            );
        }

        get_or_insert(
            node,
            NavItem {
                kind: nav::NavItemKind::Page(item),
                name: page_name.split_once('.').unwrap().0.to_string(),
            },
        );
    }

    for mystery_folder in read_dir("mysteries").unwrap().filter_map(|k| {
        let v = k.ok()?;
        v.metadata().ok()?.is_dir().then(|| v)
    }) {
        let mut nav_item_root = NavItem {
            kind: nav::NavItemKind::Folder(HashMap::new()),
            name: "root".to_string(),
        };

        let template_name = format!(
            "{}/template.html.jinja",
            mystery_folder.file_name().to_str().unwrap()
        );

        for file in recursively_search(mystery_folder.path()) {
            if file.extension() != Some(OsStr::new("md")) {
                continue;
            }

            let relative_path = file
                .strip_prefix("mysteries")
                .and_then(|e| e.strip_prefix(mystery_folder.file_name()))
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

        recursively_generate_pages(
            nav_item_root.clone(),
            //mystery_folder.path().clone(),
            &RenderingContext {
                tera: &tera,
                nav: nav_item_root,
                template_name: &template_name,
            },
        );
    }
}
