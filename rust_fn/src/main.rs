// #![feature(unboxed_closures)]
// #![feature(fn_traits)]

// FnOnce - uses what is passed in and drops it
// FnMut - it can modify it
// Fn - can take by reference : choose it
fn main() {
    let x = 2021;
    let my_closure = |num| {
        dbg!(x * num);
    };
    my_closure(2);

    let mut my_str = String::from("Let's test these closures");
    let refs_it = || println!("{}", my_str);
    refs_it();

    let mut mutes_it = || {
        my_str.push('!');
        println!("{}", my_str);
    };
    mutes_it();

    let drops_it = || drop(my_str);
    drops_it();

    let mut greeter = Greeter(String::from("Rust"));
    greeter.call();
    greeter.call_mut();
    greeter.call_once();
    // greeter.call(); -- we cannot borow self again from here since call once
    // already took ownership and self was drop since it went out of scope

    // demonstration of an area to use &mut reference
    let mut fib = Fib(2, 3);
    let res = fib.call();
    println!("Fib(2,3) >>> {}", res);

    // passing owneship and dropping
    let nonce = Nonce(vec![34, 5, 8]);
    // let arr = nonce.call(); -- allowing this move the vec
    println!("Result >>> {:?}", nonce.0);
    let arr = nonce.call();
    println!("Result >>> {:?}", arr);

    // greeter_closure();
}

struct Greeter(String);

impl Greeter {
    fn call(&self) {
        println!("Hello {}", self.0);
    }

    fn call_mut(&mut self) {
        self.call();
    }

    fn call_once(self) {
        self.call();
    }
}

struct Fib(u64, u64);

impl Fib {
    // pass self as mutable to allow mutation
    fn call(&mut self) -> u64 {
        let res = self.0;
        self.0 = self.1;
        self.1 += res;
        res
    }
}

#[derive(Debug)]
struct Nonce(pub Vec<u8>);
impl Nonce {
    fn call(self) -> Vec<u8> {
        self.0
    }
}

// reimplement the greeter struct with closure
// fn greeter_closure() {
//     let name = "Rust".to_string();
//     let mut greeter = || println!("Hello {}", name);

//     // closure is generated as a struct that implement Fn, FnOnce and FnMut traits
//     greeter();
//     greeter.call(());
//     greeter.call_mut(());
//     greeter.call_once(());
// }

struct Abstraction<T>(T);
impl<T> Abstraction<T> {
    fn map<U, F>(self, f: F) -> Abstraction<U>
    where
        F: FnOnce(T) -> U,
    {
        Abstraction(f(self.0))
    }
}
