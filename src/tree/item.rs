use std::path::PathBuf;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct TreeItem {
    pub extension: Option<String>,
    pub name: String,
    pub path: PathBuf
}

fn osstr_to_string(o: &OsStr) -> String {
    o.to_string_lossy().to_string()
}

pub fn path_name(p: &PathBuf) -> String {
    p.file_name().and_then(|e| Some(osstr_to_string(e))).unwrap()
}

impl From<PathBuf> for TreeItem {
    fn from(p: PathBuf) -> TreeItem {
        TreeItem {
            extension: p.extension().and_then(|o| {
                Some(osstr_to_string(o))
            }),
            name: path_name(&p),
            path: p
        }
    }
}