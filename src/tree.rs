use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub fn make(dir: &str) -> Result<Vec<PathBuf>, Error> {
    let mut output: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        // we'll work on symlinks later...
        if metadata.is_file() {
            output.push(path)
        } else {
            for item in make(path.to_str().unwrap())? {
                output.push(item)
            }
        }
    }
    Ok(output)
}