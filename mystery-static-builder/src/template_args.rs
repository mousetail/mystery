use serde::Serialize;

use crate::nav::NavBar;

#[derive(Serialize)]
pub struct TemplateArgs<'a> {
    parent: &'static str,
    html: &'a str,
    name: &'a str,
    nav: &'a [NavBar],
}

impl<'a> TemplateArgs<'a> {
    pub fn new(html: &'a str, name: &'a str, nav: &'a [NavBar]) -> TemplateArgs<'a> {
        TemplateArgs {
            parent: "base",
            html,
            name,
            nav,
        }
    }
}
