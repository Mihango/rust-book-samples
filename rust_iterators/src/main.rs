#![allow(unused)]


fn main() {
   test_iter();
}

// iterator pattern allows to perform task on sequence of items in turn: note that 
// iterators are lazy and take effect on consuming them
fn test_iter() {
    let v1 = vec![1, 2, 3];

    let iter = v1.iter(); // does nothing

    // loop through the iteration
    for item in iter {
        println!("Got: {}", item);
    }
}
