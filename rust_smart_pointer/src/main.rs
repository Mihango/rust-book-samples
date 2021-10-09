use crate::List::{Cons, Nil};
fn main() {
    box_sample();

    testing_linked_list();
}
///
///# Using Box to point to data on the heap. 
/// 
/// They dont have performance overhead
/// other than storing data on heap
/// 
/// Mostly used when:
/// * You have a type whose size is unknown at compile time and want to use it 
///   in a context that requires an exact size - sample recursive types
/// * Have a large amount of data and you want to transfer ownership but
///   ensure data wont be copies
/// * Want to own a value and you only care that its a tpe that implements
///   a particuler trait rather than being of specific type
fn box_sample() {
    let b = Box::new(5);
    println!("b = {}", add_two(*b));
}

fn add_two(a: i32) -> i32 {
    a + 2
}

/// cons list demonstration similar to linkedlist
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// # Using cons list (aka Linked List)
/// define an enums
/// 
/// ```
/// enum List {
///    Cons(i32, List),
///    Nil,
/// }
/// ```
/// above enum will fail with error `recursive without direction`. And suggests to use
/// some indirection (e.g `Box`, `Rc` or `&`)
///
/// use Box<T> as below
///```
///enum List {
///    Cons(i32, Box<List>),
///    Nil,
///}
///
/// let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
///```
///
fn testing_linked_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    println!("Show list : {:?}", list);
}

/// # Computing the size of a Non-recursive type
/// demostrates how sizes are calcualted in rust
///
/// To determine how much space to allocate to Message, Rust goes through
/// each variant and see which variant needs the most space. Since one variant is used, Rust
/// picks variant with most space
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}