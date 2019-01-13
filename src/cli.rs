use clap::{App, Arg};

pub fn build<'a, 'b>() -> App<'a, 'b> {
     App::new("Zarrebin")
        .version(crate_version!())
        .author(crate_authors!())
        .about("find files in terminal")
        .arg(Arg::with_name("dir")
            .takes_value(true).default_value(".").help("directory to search in"))
        .arg(Arg::with_name("extension")
            .multiple(true).takes_value(true).short("e").long("ext")
            .help("filter files by extension"))
        .arg(Arg::with_name("name")
            .takes_value(true).short("n").long("name")
            .help("filter by name"))
        .arg(Arg::with_name("depth")
            .takes_value(true).short("d").long("max-depth").help("depth for recursion (if negative, goes into every sub-directory if possible)"))
        .arg_from_usage("-S --no-stats 'disable statistics'")
        .arg_from_usage("-F --flat 'disable recursion (equivalent to --depth 0)'")
        .arg_from_usage("-H --ignore-hiddens 'ignore hidden entries'")
}