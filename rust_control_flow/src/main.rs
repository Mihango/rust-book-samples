// common constructs that let you control the flow of execution of Rust
// if expressions and loops
fn main() {
    // todo investigate returning strings println!("sample if called : {}",sample_if(4))
    sample_if(4);

    multiple_iff(32);

    _sample_loop_return_value();

    _sample_while(5);

    sample_for_loop();

    _sample_range_for();

    let degrees = convert_temp(100_f32);
    println!("{} Fahrenheit to Celcius is: {}", 100, degrees);

    let result = fibonnaci(10);
    println!("Fibonacci of 10 is: {}", result);

    let result = factorial(7);
    println!("Factorial of 7 is: {}", result);
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

// loop -- loops continuously until you stop manually
fn _sample_loop() {
    loop {
        println!("Looping Again");
    }
}

// returning value from loops
fn _sample_loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result of loop is {}", result);
}

// conditional loops with while
fn _sample_while(n: i32) {
    let mut num = n;
    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }
    println!("LIFTOFF!!!");
}

// looping through a collection with for
fn sample_for_loop() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("Value is: {}", element);
    }
}

// range
fn _sample_range_for() {
    for num in (1..5).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}

fn convert_temp(fah: f32) -> f32 {
    (fah - 32_f32) * (5_f32 / 9_f32)
}

fn fibonnaci(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
      return 1;
    }

    let mut a = 0;
    let mut b = 1;

    for _i in 2..num {
        let c = a + b;
        a = b;
        b = c;
    }

    b
 }


 fn factorial(num: i32) -> i32 {
     if num == 1 {
         return 1;
     }
     num * factorial(num - 1)
 }