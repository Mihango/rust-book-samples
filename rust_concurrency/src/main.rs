#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main() {
    simple_thread();
    move_closure_thread();
}

/// in mose os - an executed program's code run in a process
/// and os manages several processes at once
/// 2 models of creating threads:
/// - 1:1 with os - language calls os to create threads
/// - N:M language abstracts thread - own language threads are called green threads
fn simple_thread() {
    // create a new thread using thread::spawn
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("Hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /// when we move the join above the computation of main thread - it will complete
    /// first before the main thread can start -- sequential
    // handle.join().unwrap();
    for i in 1..=5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    /// above code will not wait to complete thus we need to join the handle
    /// to the main thread and the main thread will wait until all thread complete
    handle.join().unwrap();
    println!(">>>> Finished >>>>")
}

/// using move Closures with Threads
/// the move closure allows you to use data from one thread in another thread
fn move_closure_thread() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v)
    });
    handle.join().unwrap();
}
