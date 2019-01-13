use super::lib::TreeItem;

fn by_ext(i: &TreeItem, options: &[&str]) -> bool {
    if options.len() == 0 {
        true
    } else {
        match i.extension {
            Some(ref e) => options.iter().any(|x| x == e),
            None => false
        }
    }
}

pub fn by_args(i: &TreeItem, args: &clap::ArgMatches) -> bool {
    let mut res: Vec<bool> = vec![];
    if let Some(name) = args.value_of("name") {
        res.push(i.name == name)
    }

    let exts: Vec<&str> = args.values_of("extension")
        .and_then(|x| Some(x.collect())).unwrap_or(vec![]);
    res.push(by_ext(i, &exts));

    res.iter().all(|b| *b)
}