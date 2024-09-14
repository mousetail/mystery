use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct NavContext {
    source_path: PathBuf,
    destination_path: PathBuf,
    web_path: PathBuf,

    pub display_name: String,
}

impl NavContext {
    pub fn get_source_path(&self) -> &Path {
        return &self.source_path;
    }

    pub fn get_destination_path(&self) -> &Path {
        return &self.destination_path;
    }

    pub fn get_web_path(&self) -> &Path {
        return &self.web_path;
    }

    pub fn get_ancestors<'a>(
        path: &'a [String],
        mut input_root: PathBuf,
        mut output_root: PathBuf,
        mut web_root: PathBuf,
    ) -> impl Iterator<Item = NavContext> + 'a {
        return path.iter().map(move |k| {
            input_root.push(k);
            output_root.push(k);

            web_root.push(k);

            if output_root.extension().is_some() {
                output_root.set_extension("html");
                web_root.set_extension("html");
            }

            NavContext {
                source_path: input_root.clone(),
                destination_path: output_root.clone(),
                web_path: web_root.clone(),
                display_name: k.to_owned(),
            }
        });
    }
}
