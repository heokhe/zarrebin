#[macro_use]
extern crate clap;
use clap::{App, Arg};
mod tree;

fn main() {
    let args = App::new("Zarrebin")
        .version(crate_version!())
        .author("Hosein Khansari")
        .about("find files in terminal")
        .arg(Arg::with_name("dir").takes_value(true).default_value(".").help("directory to search in").index(1))
        .arg(Arg::with_name("extension").short("e").long("ext").takes_value(true).help("filter files by extension"))
        .get_matches();

    let treee = tree::make(args.value_of("dir").unwrap()).unwrap();
    println!("got {} files", treee.len())
}