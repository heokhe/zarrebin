#[macro_use]
extern crate clap;
extern crate colored;
use clap::{App, Arg};
use colored::Colorize;
mod tree;
use tree::TreeMaker;
mod filter;

fn main() {
    let args = App::new("Zarrebin")
        .version(crate_version!())
        .author(crate_authors!())
        .about("find files in terminal")
        .arg(Arg::with_name("dir")
            .takes_value(true).default_value(".").help("directory to search in"))
        .arg(Arg::with_name("extension")
            .multiple(true).takes_value(true).short("e").long("ext")
            .help("filter files by extension"))
        .arg(Arg::with_name("substring")
            .takes_value(true).short("c").long("containing")
            .help("filter files that contain the given text"))
        .arg(Arg::with_name("name")
            .takes_value(true).short("n").long("name")
            .help("filter by name"))
        .arg(Arg::with_name("depth")
            .takes_value(true).short("d").long("max-depth").help("depth for recursion (if negative, goes into every sub-directory if possible)"))
        .arg_from_usage("-F --flat 'disable recursion (equivalent to --depth 0)'")
        .arg_from_usage("-A --show-hidden 'show hidden entries'")
        .get_matches();

    let dir = args.value_of("dir").unwrap();
    let depth: isize = if args.is_present("flat") {
        0
    } else {
        args.value_of("depth").and_then(|d| d.parse().ok()).unwrap_or(-1)
    };

    let ft = TreeMaker::new(depth, dir, args.is_present("show-hidden")).make(dir);
    for result in &ft {
        match result {
            Ok(tree_item) => {
                if filter::by_args(&tree_item, &args) {
                    println!("{} Found {} {:?}", "OK:".bold().green(), tree_item.name.bold(), tree_item.path)
                }
            },
            Err(e) => {
                eprintln!("{} {}", "ERROR:".bold().red(), e.to_string())
            }
        }
    }
}