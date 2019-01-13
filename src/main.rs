#[macro_use]
extern crate clap;
extern crate colored;
use colored::Colorize;
use std::time::Instant;
mod lib;
use lib::TreeMaker;
mod cli;
mod filter;

fn main() {
    let args = cli::build().get_matches();

    let dir = args.value_of("dir").unwrap();
    let depth: isize = if args.is_present("flat") { 0 }
    else { args.value_of("depth").and_then(|d| d.parse().ok()).unwrap_or(-1) };

    let mut stats = (0,0,0);
    let instant = Instant::now();

    for result in TreeMaker::new(depth, dir, args.is_present("show-hidden")).make(dir) {
        stats.0 += 1;
        match result {
            Ok(tree_item) => {
                stats.1 += 1;
                if filter::by_args(&tree_item, &args) {
                    stats.2 += 1;
                    println!("{} Found {} {:?}", "OK:".bold().green(), tree_item.name.bold(), tree_item.path)
                }
            },
            Err((e, path)) => {
                println!("{} Processing {:?}: {} ", "ERROR:".bold().red(), path, e.to_string().bold())
            }
        }
    }
    
    if !args.is_present("no-stats") {
        let (all, no_error, passed) = stats;
        println!("{} done in {}, got {} results, {} successful, {} passed the filters", "STATS:".bold().blue(), format!("{:?}", instant.elapsed()).bold(), all, no_error, passed)
    }
}