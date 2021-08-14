// common constructs that let you control the flow of execution of Rust
// if expressions and loops
fn main() {
    // todo investigate returning strings println!("sample if called : {}",sample_if(4))
    sample_if(4);

    multiple_iff(32);
}

// if expressions - blocks of code associated with conditions are sometimes call arms
fn sample_if(num: i32) {
    if num < 5 {
        println!("Condition was true")
    } else {
        println!("Condition was false")
    }
}

// multiple if expression
fn multiple_iff(num: i32) {
    if num % 4 == 0 {
        println!("{} is divisible by 4", num)
    } else if num % 3 == 0 {
        println!("{} is divisible by 3", num)
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
