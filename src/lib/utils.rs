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

/// gets the "real path" of a PathBuf (like unix realpath)
pub fn realpath(target: &PathBuf, base: &PathBuf) -> PathBuf {
	PathBuf::from(
		path_to_string(&target.canonicalize().unwrap())
			.replacen(&path_to_string(&base.canonicalize().unwrap()), "", 1)
			.replacen('/', "", 1)
	)
}

pub fn path_to_string(p: &PathBuf) -> String {
	format!("{:?}", p).replace('"', "")
}