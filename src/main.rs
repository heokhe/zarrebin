#[macro_use]
extern crate clap;
extern crate colored;
use colored::Colorize;
use std::time::Instant;
mod lib;
use lib::{TreeMaker, path_to_string};
mod cli;
use std::env;
mod filter;

fn main() {
    let args = cli::build().get_matches();
    
    let no_pretty = args.is_present("machine");
    if no_pretty { colored::control::set_override(false) }

	let cur_dir = path_to_string(&env::current_dir().unwrap());
    let dir = args.value_of("dir").unwrap_or(&cur_dir);
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
                        println!("{} {}", "found".green().bold(), path_to_string(&item.path).bold())
                    }
                }
            },
            Err((err, path)) => {
                if !args.is_present("ignore-errors") {
                    println!("{} {}: {} ", "error".bold().red(), err.to_string().to_lowercase().bold(), path_to_string(&path))
                }
            }
        }
    }
    
    if !args.is_present("no-stats") {
        let (all, no_error, passed) = stats;
        let dur = instant.elapsed();
        println!("{} done in {}, got {} results, {} successful, {} passed the filters", "stats".bold().blue(), format!("{:?}", dur).bold(), all, no_error, passed)
    }
}