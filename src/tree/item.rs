use std::path::PathBuf;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct TreeItem {
    pub extension: Option<String>,
    pub name: String,
    pub path: PathBuf
}

impl From<PathBuf> for TreeItem {
    fn from(p: PathBuf) -> TreeItem {
        let osstr_to_string = |e: &OsStr| Some(e.to_string_lossy().to_string());
        TreeItem {
            extension: p.extension().and_then(osstr_to_string),
            name: p.file_name().and_then(osstr_to_string).unwrap(),
            path: p
        }
    }
}