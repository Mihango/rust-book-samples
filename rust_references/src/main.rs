#[allow(unused)]
// Refrences and borrowing

// References - feature that allow use of variables without transfer of ownership
// borrowing - references as function parameters
fn main() {
    let s1 = String::from("Hello");
    let (s1, len) = calculate_length_tuple(s1);
    println!("The length of {} is: {}", s1, len); // using s1 insted of s2 throws compile error due to
                                                  // scope

    let s3 = String::from("Hello");
    let size = calulate_length_reference(&s3); // borrows ownership

    // mutating borrowed string
    let mut s4 = String::from("Hello");
    mutate_string(&mut s4); // have to add mut at & and also make the string mutable
    println!("Mutated string result >>> {}", s4);
}

// challenge with this code is ownership is transferred and then you have to return the String
fn calculate_length_tuple(str: String) -> (String, usize) {
    let length = str.len(); // len returns length of a string
    (str, length)
}

// has a refrence to a String - the & are references and they allow you to refer to some value without taking
// ownership of it
fn calulate_length_reference(str: &String) -> usize {
    str.len()
} // str will not be dropped when it goes out of scope since it does not have ownership of what it refers to

// mutable refrences - NB: you can only have one mutable reference to a particular piece of data in
// a particular scope
fn mutate_string(str: &mut String) {
    str.push_str(", World");
}
