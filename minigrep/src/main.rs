#![allow(unused)]

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // get arguments using std::args function which returns an iterator
    // call iterator's collect method turns the data produced to a collection
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parging arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {} \n", config.filename);

    if let Err(er) = minigrep::run(config) {
        eprintln!("Application error: {}", er);
        process::exit(1);
    }
}