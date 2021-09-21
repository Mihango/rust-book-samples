#![allow(unused)]

fn main() {
    // sample code to test generic lifetimes with comparing length of strings
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest sring is {}", result);

    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// every reference has a lifetime, which is the scope for which that reference is valid
// lifetimes are implicit or inferred
// as types can be of multiple types so is the case with lifetimes:  Rust requires we annotate
// the relationships using generic lifetime parameters to ensure the actual refrerences used at runtime will definitely be valid


// them main aim of lifetimes is to prevent dangling references which can cause a program to refrence data 
// other than the d ata its intended to reference: conside below code with outer and inner scopes: it fails to compile
fn demo_scopes() {
    let r = 0; // remove 0

    // inner scope
    // {
    //     let x = 5;
    //     r = &x;
    // } // x does not live enough: drops here while still borrowd

    println!("r : {}", r);
}

// rust determines the validity of above code using the borrow-checker mechanism


// generic lifetimes in functions

// takes in string slices, which are refernces so longest doesnt take ownership of parameters
// Rust cant determine whether the reference being returned refers to x or y:  - the borrow checker can either since it doesnt
// know  how lifetimes of x and y relate to the lifetime of the returned values. -- 

// fn longest(x: &str, y: &str) -> &str {
//     if(x.len() > y.len()) {
//         x
//     } else {
//         y
//     }
// }

// to fix this we add lifetime paremeters that define the relationship btn the references so the borrow checker can perform analysis
// syntax
// 1. &i32 - reference
// 2. &'a i32 - a reference with explicit lifetime
// 3. &'a mut i32 - a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

