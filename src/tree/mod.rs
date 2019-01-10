use std::fs;
use std::io::Error;
use std::path::PathBuf;
mod item;
pub use self::item::TreeItem;

pub type Tree = Vec<TreeItem>;

/// returns the "length" of a path.
fn path_len(p: &PathBuf) -> usize {
    p.components().collect::<Vec<_>>().len()
}

pub struct TreeMaker {
    max_depth: isize,
    root_len: usize
}

impl TreeMaker {
    pub fn new(max_depth: isize, root_dir: &str) -> TreeMaker {
        TreeMaker { max_depth, root_len: path_len(&PathBuf::from(root_dir)) }
    }
    pub fn make(&self, dir: &str) -> Result<Tree, Error> {
        let root_len = self.root_len;
        let max_depth = self.max_depth;
        let mut output: Tree = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            if metadata.is_file() {
                output.push(TreeItem::from(path))
            } else if metadata.is_dir() {
                let cur_depth = path_len(&path) - root_len - 1;

                let should_pass = if max_depth < 0 {
                    true
                } else {
                    cur_depth < self.max_depth as usize
                };
                if should_pass {
                    for item in self.make(path.to_str().unwrap())? {
                        output.push(item)
                    }
                }
            } else {
                // we'll work on symlinks later...
            }
        }
        Ok(output)
    }
}