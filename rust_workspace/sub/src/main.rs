use add_one;
fn main() {
    println!("Running random");
    let num = 32;
    let result = add_one::random::add_random(num);
    println!("{} + {} = {}", num, result.0, result.1)
}
