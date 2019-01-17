#[macro_use]
extern crate clap;
extern crate colored;
use colored::Colorize;
use std::time::Instant;
mod lib;
use lib::{TreeMaker, path_to_string};
mod cli;
mod filter;

fn main() {
    let args = cli::build().get_matches();
    
    let no_pretty = args.is_present("machine");
    if no_pretty { colored::control::set_override(false) }

    let dir = args.value_of("dir").unwrap();
    let depth: isize = if args.is_present("flat") { 0 }
    else { args.value_of("depth").and_then(|d| d.parse().ok()).unwrap_or(-1) };
    let exclude = args.values_of("exclude").and_then(|x| Some(x.collect())).unwrap_or(vec![]);

    let mut stats = (0,0,0);
    let instant = Instant::now();
    for result in TreeMaker::new(depth, dir, args.is_present("ignore-hiddens"), &exclude).make(dir) {
        stats.0 += 1;
        match result {
            Ok(item) => {
                stats.1 += 1;
                if filter::by_args(&item, &args) {
                    stats.2 += 1;
                    if no_pretty {
                        println!("{}", path_to_string(&item.path))
                    } else {
                        println!("{} found {}", "OK:".bold().green(), path_to_string(&item.path).bold())
                    }
                }
            },
            Err((err, path)) => {
                if !args.is_present("ignore-errors") {
                    println!("{} processing {:?}: {} ", "ERROR:".bold().red(), path, err.to_string().bold())
                }
            }
        }
    }
    
    if !args.is_present("no-stats") {
        let (all, no_error, passed) = stats;
        let dur = instant.elapsed();
        println!("{} done in {}, got {} results, {} successful, {} passed the filters", "STATS:".bold().blue(), format!("{:?}", dur).bold(), all, no_error, passed)
    }
}