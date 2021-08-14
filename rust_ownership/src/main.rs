#![allow(unused)]
// Rust does memory management through a system of ownership with a set of rules that the
// compiler checks at compiler time

// STACK - All data stored on stack must have a known, fixed size
// HEAP - Stores data with varying size and is slow as compared to stack

// Ownership rules
// 1. Each value in Rust has a variable that's called an owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn main() {
    sample_string();

    ownership_clone();

    ownership_stack_copy();
}

// variable scope : scope is the range within a program for which an item is valid
fn sample_variable_scope() {
    let s = "Hello"; // string literal - and are immutable
}

// string type - not literal: mutable
fn sample_string() {
    // :: operator allows to namespace this partcular from fuction under the String type
    let mut s = String::from("Hello"); // type is allocated on the heap

    // s can be mutated
    s.push_str(", world!"); // push_str appends a literal to a String

    println!("{}", s);
}

// ways variables and data interact: mutliple variables can interact with the same data in different ways
// 1. Move
fn ownership_move() {
    let x = 5;
    let y = x; // makes a copy of x and bind it to y

    let s1 = String::from("Hello");
    let s2 = s1; // s1 string data(i.e pointer, length, capacity) which is stored in stack is copied to s2
                 // but not the data in the heap

    // NB - when variable goes out of scope, Rust calls drop function to clean up heap memory
    // for that variable - this might cause a problem if both s2 and s1 goes out of scop as they
    // both try to free same memory...this is known as doouble free error - which may lead to
    // memory corruption

    // To ensure memory safety, instead of trying to copy the allocated memory, Rust considers
    // s1 to no longer be valid and therefor Rust doesnt need to free anything when s1 goes out
    // of scope

    // below leads to a bug
    // println!("{}, world!", s1); // s1 is invalid and compiler complains - err value borrowed
    // here after move

    // shallow copy is similar to move, but Rust also invalidated the first variable - with this
    // it implies Rust will never automaticall create deep copies of data
}

// 2. Clone - used to do deep copy of data
fn ownership_clone() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // is expensive - a new copy is created with different pointer

    println!("s1 = {}, s2 = {}", s1, s2);
}

// 3. Copy - stack only data -- also Rust has Copy trait - annotation to store
fn ownership_stack_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // this contradict above rules on ownership...what makes
                                      // it different where x is still valid and was never moved to y?
                                      // reason: all variables with known size at compile time are stored
                                      // entirely on the stack.- where making copies are quick
}

// ownership and functions
fn function_ownership() {
    let s = String::from("Hello");
    takes_ownership(s); //s's value moves into the function and so is no longer valid below here

    // println!("{}", s); <-- calling s here will result to error as it has already been dropped -->

    let x = 5;
    makes_copy(x); // x wold move into function, but i32 is Copy, so its okay to still use x afterwards
}

fn takes_ownership(str: String) { // str comes into scope
    println!("{}", str);
} // str goes out of scope and drop is called - the backing  memeory is freed

fn makes_copy(int: i32) { // int comes into scope
    println!("copied: {}", int)
} // int goes out of scope - nothing happens

// return values and scope - they can also transfer ownership
fn takes_and_gives_owneship(str: String) -> String {
    str
}
