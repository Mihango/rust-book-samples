use std::fmt::Display;

fn main() {
    // immutability
    let x = 5;
    println!("the value x is : {}", x);
    // x = 6;  cannot work until you make x mutable

    let mut y = 5;
    println!("the value y is : {}", y);
    y = 10;
    println!("the value y is : {}", y);

    // constants
    const Z: u32 = 10;
    println!("The value of z is : {}", Z);

    // shadowing
    let a = 23;
    println!("The value of a is: {}", a);

    let a = a + 32;
    println!("The value of a is: {}", a);

    let a = 56;
    println!("The value of a is: {}", a);

    let a = "Hello"; // with shadowing you can change the type
    println!("The value of a is: {}", a);


    // data types - scalar types - integer: 
    // i8, u8, i16, u16, i32, u32, i64, u64, isize, usize
    let guess: u32 = "42".parse().expect("Not a number");
    println!("The value of guess : {}", guess);
    let guess: i32 = -32;
    println!("The value of guess : {}", guess);

    // float - f32, f64
    let x = 2.0; // double precision
    println!("the value x is : {}", x);


    let x: f32 = -3.0; // single precision
    println!("the value x is : {}", x);

    // numeric expression -- starting variables with _ removes unused warning
    let _sum = 5+10; 
    let _diff = 95.5 - 4.3;
    let _prod = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _remainder = 43 % 5;

    // boolean type
    let _t = true;
    let _f: bool = false;

    // character type
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("character values: c = {} , Z = {}, heart = {}", c, z, heart_eyed_cat);


    // compound types
    // 1. tuple -- can have different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("Value of tuple is : {}, {}, {}, y = {}", tup.0, tup.1, tup.2, y);

    // 2. Array -- must have same type
    let _arr = [1, 2, 3, 4, 5]; 
    let _arr: [i32; 5];
    _arr = [5, 6, 7, 8, 9];
    let _arr = [3; 25];
    

}
