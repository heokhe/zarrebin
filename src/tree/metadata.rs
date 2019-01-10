use std::io::Error;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    File, Directory, Symlink
}

fn type_of_entry(e: &std::fs::DirEntry) -> Result<FileType, Error> {
    let md = e.metadata()?;
    Ok({
        if md.is_file() { FileType::File }
        else if md.is_dir() { FileType::Directory}
        else { FileType::Symlink }
    })
}

pub fn info(entry: DirEntry) -> Result<(PathBuf, FileType), Error> {
    let p = entry.path();
    type_of_entry(&entry).and_then(|md| Ok((p, md)))
}