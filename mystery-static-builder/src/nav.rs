use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::{NavContext, NavItem};

#[derive(Serialize, Debug)]
pub struct NavBar {
    items: Vec<NavBarItem>,
}

#[derive(Serialize, Debug)]
struct NavBarItem {
    selected: bool,
    web_path: PathBuf,
    display_name: String,
}

fn generate_nav_frame(items: &[NavItem], root: &str) -> Vec<NavBarItem> {
    items
        .iter()
        .map(|item| NavBarItem {
            selected: item.path.get_display_name() == root,
            web_path: item.path.get_web_path().to_owned(),
            display_name: item.path.get_display_name().to_owned(),
        })
        .collect()
}

pub fn generate_nav_bar(items: &[NavItem], current_page: &NavContext) -> Vec<NavBar> {
    current_page
        .get_display_path()
        .iter()
        .scan(items, |items, frame| {
            let out = generate_nav_frame(items, frame);

            match &items
                .iter()
                .find(|k| k.path.get_display_name() == frame)
                .unwrap_or_else(|| panic!("Frame: {:?}", frame))
                .kind
            {
                crate::NavItemKind::Page(_page) => (),
                crate::NavItemKind::Folder(vec) => *items = vec.as_slice(),
            };

            Some(NavBar { items: out })
        })
        .collect()
}
