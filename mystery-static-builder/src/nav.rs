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
pub struct NavBarItem {
    selected: bool,
    path: NavContext,
}

fn generate_nav_frame(items: &[NavItem], root: &OsStr) -> Vec<NavBarItem> {
    items
        .iter()
        .map(|item| NavBarItem {
            selected: item.path.display_name.as_bytes() == root.as_encoded_bytes(),
            path: item.path.clone(),
        })
        .collect()
}

pub fn generate_nav_bar(
    items: &[NavItem],
    current_page: &NavContext,
    web_prefix: &Path,
) -> Vec<NavBar> {
    current_page
        .get_web_path()
        .parent()
        .unwrap()
        .join(&current_page.display_name)
        .strip_prefix(web_prefix)
        .unwrap()
        .iter()
        .scan(items, |items, frame| {
            let out = generate_nav_frame(items, frame);

            match &items
                .iter()
                .find(|k| k.path.display_name.as_bytes() == frame.as_encoded_bytes())
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
