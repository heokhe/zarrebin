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
        .arg(Arg::with_name("dir")
            .takes_value(true).default_value(".").help("directory to search in"))
        .arg(Arg::with_name("extension")
            .multiple(true).takes_value(true).require_delimiter(true).short("e").long("ext")
            .help("filter files by extension (seperated by commas)"))
        .arg(Arg::with_name("substring")
            .takes_value(true).short("c").long("containing")
            .help("filter files that contain the given text"))
        .arg(Arg::with_name("name")
            .takes_value(true).short("n").long("name")
            .help("filter by name"))
        .get_matches();

    let dir = args.value_of("dir").unwrap();
    let treee = tree::make(dir).unwrap();
    for item in &treee {
        if filter::by_args(&item, &args) {
            println!("{}", item.path.to_str().unwrap())
        }
    }
}