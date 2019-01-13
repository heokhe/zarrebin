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

    for result in TreeMaker::new(depth, dir, args.is_present("ignore-hiddens")).make(dir) {
        stats.0 += 1;
        match result {
            Ok(item) => {
                stats.1 += 1;
                if filter::by_args(&item, &args) {
                    stats.2 += 1;
                    println!("{} found {} {:?}", "OK:".bold().green(), item.name.bold(), item.path)
                }
            },
            Err((err, path)) => println!("{} processing {:?}: {} ", "ERROR:".bold().red(), path, err.to_string().bold())
        }
    }
    
    if !args.is_present("no-stats") {
        let (all, no_error, passed) = stats;
        let dur = instant.elapsed();
        println!("{} done in {}, got {} results, {} successful, {} passed the filters", "STATS:".bold().blue(), format!("{:?}", dur).bold(), all, no_error, passed)
    }
}