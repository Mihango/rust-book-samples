fn main() {
    println!("Hello, world!");
    another_func(32);

    let x = add_nums(34, 54);
    println!("Value of x is {}", x);

    _am_expression();

    let five = _five();
    println!("The value returned by _five() call is : {}", five);
}

fn another_func(x: i32) {
    println!("Another function --> value of x is : {}", x);
}

// function bodies are made up of a series of statements optionally ending
// in an expression
// statement -> instruction the perfom same action and dont return value
// expression -> evaluates to a resulting value
fn add_nums(x: i32, y: i32) -> i32 {
    y + x // its an expression since has no semi colon -> return type i32
}
// statement example - function definitions are also statements
// statements dont return values hence you can't assign a let to another variable
fn _am_statement() {
    let _y = 6;
}

// expressiong evaluate to something eg 5 + 6 is expressiong evaluating to 111
// calling a function/macro, {} == used to create new scopes are expressions
fn _am_expression() {
    let x = 5;

    // new scope for x - where below is expression - expressions dont include ; which turns them into
    // a statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The values of x and y are : {} , {}", x, y);
}

// functions with return values: use -> : return value is synonymous with the value of the final
// expression in the block of the body of a fn -- can also return early using return keyword
fn _five() -> i32 {
    5
}
