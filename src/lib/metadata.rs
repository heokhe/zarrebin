use std::io::Error;
use std::fs::DirEntry;
use std::path::PathBuf;

pub enum FileType {
    File, Directory, Symlink
}

fn type_of_entry(e: &DirEntry) -> Result<FileType, Error> {
    let md = e.metadata()?;
    Ok({
        if md.is_file() { FileType::File }
        else if md.is_dir() { FileType::Directory}
        else { FileType::Symlink }
    })
}

pub fn info(entry: DirEntry) -> Result<(PathBuf, FileType), (Error, PathBuf)> {
    let p = entry.path();
    match type_of_entry(&entry) {
        Ok(ft) => Ok((p, ft)),
        Err(e) => Err((e, p))
    }
}