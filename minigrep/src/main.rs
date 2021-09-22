#![allow(unused)]

use std::env;

fn main() {
    // get arguments using std::args function which returns an iterator
    // call iterator's collect method turns the data produced to a collection
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
