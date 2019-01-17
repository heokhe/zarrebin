use std::fs;
use std::io::Error;
use std::path::PathBuf;
mod item;
mod metadata;
mod utils;
pub use self::utils::*;
pub use self::item::TreeItem;
use self::metadata::FileType;

pub struct TreeMaker<'a> {
    max_depth: isize,
	root_dir: PathBuf,
    ignore_hiddens: bool,
    exclude: &'a[&'a str],
}

impl<'a> TreeMaker<'a> {
    pub fn new(max_depth: isize, root_dir: &'a str, ignore_hiddens: bool, exclude: &'a[&str]) -> TreeMaker<'a> {
        TreeMaker {
            max_depth, exclude, ignore_hiddens, 
			root_dir: PathBuf::from(root_dir)
        }
    }
    pub fn make(&self, dir: &str) -> Vec<Result<TreeItem, (Error, PathBuf)>> {
        let root_len = path_len(&self.root_dir);
        let max_depth = self.max_depth;
        let mut stack = vec![];
        if let Ok(entries) = fs::read_dir(fs::canonicalize(dir).unwrap()) {
            for entry in entries.map(|e| e.unwrap()) {
                match metadata::info(entry) {
                    Ok((path, ftype)) => {
                        if self.ignore_hiddens && path_name(&path).starts_with(".") { continue }

                        match ftype {
                            FileType::File => {
								stack.push(Ok(TreeItem::from(path)))
							},
                            FileType::Directory => {
                                let cur_depth = path_len(&path) - root_len - 1;

                                let should_pass = if max_depth < 0 {
                                    true
                                } else {
                                    cur_depth < self.max_depth as usize
                                };
                                let is_excluded = self.exclude.iter().any(|e| *e == path_name(&path));

                                if should_pass && !is_excluded {
                                    stack.append(&mut self.make(&path_to_string(&path)))
                                }
                            }
                            FileType::Symlink => continue, // TODO: add support for symlinks
                        }
                    },
					Err(e) => stack.push(Err(e))
                }
            }
        }

        stack
    }
}
