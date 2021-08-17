#![allow(unused)]
fn main() {
    // create a new vector using macro -- list
    let mut countries = vec!["Kenya", "Uganda", "Tanzania"];
    countries.push("Somalia");

    // reading elements
    let first: &str = &countries[0]; // returns a reference

    // use get to read .. which is safe than [] -- handles nullability
    match countries.get(0) {
        Some(first) => println!("Display country 1 : {}", first),
        None => println!("No first element"),
    }

    // create mutable vector with new
    let mut cities: Vec<String> = Vec::new();
    cities.push(String::from("Nairobi"));
    cities.push(String::from("Kisumu"));
    cities.push(String::from("Nakuru"));

    // ownership rule - cannot have a mutable and immutable references in the same scope
    // below will not compile if uncommented
    let city: &str = &cities[0]; // -- move have occurred here since there is borrowing - immutable

    cities.push(String::from("Mombasa")); // then push -- allowed until below -- mutable borrow occurs here

    // println!("City gotten >>> {}", city); // immutable borrow used here -- this is prevented since the
                                          // memory referenced to might have been deallocated


    // reading again is allowed
    println!("City 0 >>> {}", &cities[0]);

    // iterating throught items in vector collection
    for city in &cities {
        println!("City >>> {}", city);
    }


    // iterate and make changes == use deference operator(*)
    let mut nums = vec![100, 32, 57];
    for i in &mut nums {
        *i += 50;

        println!("Num >>> {}", i);
    }
}
