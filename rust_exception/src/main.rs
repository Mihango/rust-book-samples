#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // handle_revorable_error();
    handle_revorable_error_kind();
}

// this functions causes panic and stop the program
fn cause_panic() {
    // cause panic with panic! macro
    panic!("shutting down now...");
}

// this is recovarable type error
fn handle_revorable_error() {
    // does not cause panic
    let f = File::open("hello.txt"); // f returns Result enum type

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// matching on different errors - taking different action on different failure reasons
fn handle_revorable_error_kind() {
    let f = File::open("hello.txt");

    // the matches are many can be solved using closures
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
