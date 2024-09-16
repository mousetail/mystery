use std::{
    ffi::OsStr,
    fs::{read_dir, DirEntry, OpenOptions},
    io::{Error, Read},
    path::{Path, PathBuf},
};

use nav::generate_nav_bar;
use path_context::NavContext;
use serde::Serialize;
use template_args::TemplateArgs;

mod nav;
mod path_context;
mod template_args;

const WEB_ROOT: &'static str = "/";
const SOURCE_ROOT: &'static str = "mysteries";
const DESINATION_ROOT: &'static str = "frontend-build";

fn os_str_starts_with(string: &OsStr, prefix: &str) -> bool {
    string.as_encoded_bytes().get(..prefix.len()) == Some(prefix.as_bytes())
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

fn read_nav<'a>(nav: &'a Vec<NavItem>) -> Box<dyn Iterator<Item = &'a NavContext> + 'a> {
    fn _inner<'a>(nav: &'a Vec<NavItem>) -> Box<dyn Iterator<Item = &'a NavContext> + 'a> {
        return Box::new(nav.iter().flat_map(move |k| match &k.kind {
            NavItemKind::Page(_page) => Box::new(Some(&k.path).into_iter()),
            NavItemKind::Folder(vec) => _inner(vec),
        }));
    }
    _inner(nav)
}

#[derive(Debug)]
struct Page {}

#[derive(Debug)]
enum NavItemKind {
    Page(Page),
    Folder(Vec<NavItem>),
}

#[derive(Debug)]
struct NavItem {
    kind: NavItemKind,
    path: NavContext,
}

fn get_or_insert<'a>(
    list: &'a mut Vec<NavItem>,
    value: NavContext,
    kind: NavItemKind,
) -> Option<&'a mut Vec<NavItem>> {
    if !list
        .iter()
        .any(|k| k.path.get_display_name() == value.get_display_name())
    {
        list.push(NavItem {
            kind: kind,
            path: value.clone(),
        });
    }

    let new_nav_item = list
        .iter_mut()
        .find(|child| child.path.get_display_name() == value.get_display_name())
        .unwrap();
    return match &mut new_nav_item.kind {
        NavItemKind::Folder(k) => Some(k),
        NavItemKind::Page(_) => None,
    };
}

fn main() {
    let web_root = Path::new(WEB_ROOT);
    let destination_root = Path::new(DESINATION_ROOT);
    let source_root = Path::new(SOURCE_ROOT);

    let mut handlebars = handlebars::Handlebars::new();

    let base_template = PathBuf::from("mysteries/base.hbs");
    handlebars
        .register_template_file("base", base_template)
        .unwrap();

    fn insert_page(
        root: &mut Vec<NavItem>,
        item: Page,
        path: Vec<String>,
        input_root: &Path,
        output_root: &Path,
        web_root: &Path,
    ) {
        let mut node = Some(root);

        let values = (&path[..path.len() - 1])
            .iter()
            .map(|_| NavItemKind::Folder(vec![]))
            .chain(Some(NavItemKind::Page(item)));

        for (folder, kind) in NavContext::get_ancestors(
            &path,
            input_root.to_owned(),
            output_root.to_owned(),
            web_root.to_owned(),
            vec![],
        )
        .zip(values)
        {
            node = get_or_insert(node.unwrap(), folder, kind);
        }
    }

    for mystery_folder in read_dir(source_root).unwrap().filter_map(|k| {
        let v = k.ok()?;
        v.metadata().ok()?.is_dir().then(|| v)
    }) {
        let stripped_mystery_folder = mystery_folder
            .path()
            .strip_prefix(source_root)
            .unwrap()
            .to_owned();

        let source_root = source_root.join(&stripped_mystery_folder);
        let destination_root = destination_root.join(&stripped_mystery_folder);
        let web_root = web_root.join(&stripped_mystery_folder);

        let mut nav_item_root = vec![];

        let template = mystery_folder.path().join("./template.hbs");
        let template_name = mystery_folder.file_name().to_str().unwrap().to_string();
        handlebars
            .register_template_file(&template_name, template)
            .unwrap();

        for file in recursively_search(mystery_folder.path()) {
            eprintln!(
                "Discovered file {:?} {source_root:?} {destination_root:?} {web_root:?}",
                file
            );

            if file.extension() != Some(OsStr::new("md")) {
                continue;
            }

            let relative_path = file
                .strip_prefix(&source_root)
                .unwrap()
                .components()
                .map(|d| d.as_os_str().to_str().unwrap().to_owned())
                .collect();

            insert_page(
                &mut nav_item_root,
                Page {},
                relative_path,
                &source_root,
                &destination_root,
                &web_root,
            );
        }

        for page in read_nav(&nav_item_root) {
            eprintln!(
                "Processing file {:?} {:?}",
                page,
                page.get_destination_path()
            );

            let mut text = String::new();
            OpenOptions::new()
                .read(true)
                .write(false)
                .open(page.get_source_path())
                .expect("Failed to open file")
                .read_to_string(&mut text)
                .expect("Failed to read file content");

            let nav_bar = generate_nav_bar(&nav_item_root, &page);

            let html = markdown_extensions::render_markdown(&text);

            let output_file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(page.get_destination_path())
                .unwrap();

            handlebars
                .render_to_write(
                    &template_name,
                    &TemplateArgs::new(&html, &page.get_display_name(), &nav_bar),
                    output_file,
                )
                .unwrap();
        }
    }
}
