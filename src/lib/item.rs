use std::path::PathBuf;
use super::utils::{path_name, osstr_to_string};

#[derive(Debug)]
pub struct TreeItem {
    pub extension: Option<String>,
    pub name: String,
    pub path: PathBuf
}

impl From<PathBuf> for TreeItem {
    fn from(p: PathBuf) -> TreeItem {
        TreeItem {
            extension: p.extension().and_then(|o| {
                Some(osstr_to_string(o))
            }),
            name: path_name(&p), path: p
        }
    }
}