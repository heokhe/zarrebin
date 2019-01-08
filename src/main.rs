#[macro_use]
extern crate clap;
use clap::{App, Arg};
mod tree;
mod filter;

fn main() {
    let args = App::new("Zarrebin")
        .version(crate_version!())
        .author("Hosein Khansari")
        .about("find files in terminal")
        .arg(Arg::with_name("dir").takes_value(true).default_value(".").help("directory to search in").index(1))
        .arg(Arg::with_name("extension").multiple(true).short("e").long("ext").takes_value(true).help("filter files by extension"))
        .get_matches();

    let dir = args.value_of("dir").unwrap();
    let treee = tree::make(dir).unwrap();
    let exts: Vec<&str> = args.values_of("extension")
        .and_then(|x| Some(x.collect())).unwrap_or(vec![]);
    
    for item in &treee {
        if filter::by_ext(&item, &exts) {
            println!("{:#?}", item)
        }
    }

    println!("got {} files", treee.len())
}