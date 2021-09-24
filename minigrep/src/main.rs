#![allow(unused)]

use std::env;
use std::fs;

fn main() {
    // get arguments using std::args function which returns an iterator
    // call iterator's collect method turns the data produced to a collection
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {} \n", filename);

    let contents =
        fs::read_to_string(filename).expect("Something went wrong while reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
