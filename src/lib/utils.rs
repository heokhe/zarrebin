use std::ffi::OsStr;
use std::path::PathBuf;

/// converts an OsStr to String.
pub fn osstr_to_string(o: &OsStr) -> String {
    o.to_string_lossy().to_string()
}

/// returns the final component of a path (file or directory name).
pub fn path_name(p: &PathBuf) -> String {
    p.file_name()
        .and_then(|e| Some(osstr_to_string(e)))
        .unwrap_or(String::from(""))
}

/// returns the "length" of a path.
pub fn path_len(p: &PathBuf) -> usize {
    p.components().collect::<Vec<_>>().len()
}