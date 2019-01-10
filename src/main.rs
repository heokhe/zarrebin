#[macro_use]
extern crate clap;
use clap::{App, Arg};
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
            .takes_value(true).short("d").long("max-depth").help("depth for recursion (if negative, goes into every sub-directory if possible"))
        .arg_from_usage("-F --flat 'disable recursion (equivalent to --depth 0)'")
        // .arg(Arg::with_name("exclude")
        //     .takes_value(true).multiple(true).short("X").long("exclude")
        //     .help("exclude directories"))
        .get_matches();

    let depth: isize = if args.is_present("flat") {
        0
    } else {
        args.value_of("depth").and_then(|d| d.parse().ok()).unwrap_or(-1)
    };

    let dir = args.value_of("dir").unwrap();
    match TreeMaker::new(depth, dir).make(dir) {
        Ok(file_tree) => {
            for item in &file_tree {
                if filter::by_args(&item, &args) {
                    println!("{}", item.path.to_str().unwrap())
                }
            }
        },
        Err(e) => {
            println!("{}", e.to_string())
        }
    }
}