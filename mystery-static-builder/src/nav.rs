use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct NavItem {
    pub name: String,
    #[serde(flatten)]
    pub kind: NavItemKind,
}

#[derive(Clone, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum NavItemKind {
    Page(Page),
    Folder(HashMap<String, NavItem>),
}

#[derive(Clone, Serialize)]
pub struct Page {
    pub markdown_location: PathBuf,
}

pub fn get_or_insert<'a>(list: &'a mut NavItem, value: NavItem) -> &'a mut NavItem {
    match &mut list.kind {
        NavItemKind::Page(_page) => panic!("Attempted to insert sub-page under page"),
        NavItemKind::Folder(hash_map) => hash_map.entry(value.name.clone()).or_insert(value),
    }
}
