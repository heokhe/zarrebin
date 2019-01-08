use super::tree::TreeItem;

pub fn by_ext(i: &TreeItem, options: &[&str]) -> bool {
    match i.extension {
        Some(ref e) if options.len() > 0 => {
            options.iter().any(|x| x == e)
        },
        _ => false
    }
}