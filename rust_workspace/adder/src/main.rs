use add_one;
fn main() {
    let x = 23;
    let value = add_one::add_one_num(x);
    println!("{} + 1 = {}", x, value);
}
