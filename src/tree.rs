use std::fs;
use std::io::Error;
use std::path::PathBuf;
use std::ffi::OsStr;

#[derive(Debug)]
pub struct TreeItem {
    pub extension: Option<String>,
    pub name: String
}

impl From<PathBuf> for TreeItem {
    fn from(p: PathBuf) -> TreeItem {
        let osstr_to_string = |e: &OsStr| Some(e.to_string_lossy().to_string());
        TreeItem {
            extension: p.extension().and_then(osstr_to_string),
            name: p.file_name().and_then(osstr_to_string).unwrap()
        }
    }
}

pub type Tree = Vec<TreeItem>;

pub fn make(dir: &str) -> Result<Tree, Error> {
    let mut output: Tree = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        // we'll work on symlinks later...
        if metadata.is_file() {
            output.push(TreeItem::from(path))
        } else {
            for item in make(path.to_str().unwrap())? {
                output.push(TreeItem::from(item))
            }
        }
    }
    Ok(output)
}