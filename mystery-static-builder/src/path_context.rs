use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct NavContext {
    source_path: PathBuf,
    destination_path: PathBuf,
    web_path: PathBuf,
    display_path: Vec<String>,
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

    pub fn get_display_path(&self) -> &[String] {
        return &self.display_path;
    }

    pub fn get_display_name(&self) -> &str {
        return self.display_path.last().unwrap();
    }

    fn to_destination_path(filename: &str) -> String {
        return Self::strip_extension(filename)
            .map(|k| {
                let mut s = k.to_owned();
                s.push_str(".html");
                return s;
            })
            .unwrap_or_else(|| filename.to_owned());
    }

    fn strip_extension(filename: &str) -> Option<&str> {
        if let Some((a, _b)) = filename.rsplit_once(".") {
            Some(a)
        } else {
            None
        }
    }

    fn maybe_strip_extension(filename: &str) -> &str {
        return Self::strip_extension(filename).unwrap_or(filename);
    }

    pub fn get_ancestors<'a>(
        path: &'a [String],
        mut input_root: PathBuf,
        mut output_root: PathBuf,
        mut web_root: PathBuf,
        mut display_root: Vec<String>,
    ) -> impl Iterator<Item = NavContext> + 'a {
        return path.iter().map(move |input_path_segment| {
            let output_path_segment = Self::to_destination_path(input_path_segment);
            let web_path_segment = &output_path_segment;
            let display_path_segment = Self::maybe_strip_extension(&input_path_segment);

            input_root.push(input_path_segment);
            output_root.push(output_path_segment.to_owned());
            display_root.push(display_path_segment.to_owned());
            web_root.push(web_path_segment);

            if output_root.extension().is_some() {
                output_root.set_extension("html");
                web_root.set_extension("html");
            }

            NavContext {
                source_path: input_root.clone(),
                destination_path: output_root.clone(),
                web_path: web_root.clone(),
                display_path: display_root.clone(),
            }
        });
    }
}
