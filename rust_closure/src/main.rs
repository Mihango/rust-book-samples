#![allow(unused)]

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let intensity = simulated_expensive_calculation(43);
    println!("Intensity returned >>>> {}", intensity);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// generating workout which calls several expensive calculation -- to be simplified later
// to ensure optimization -- memoization / lazy evaluation
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today!, Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout_2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result);
        println!("Next, do {} situps", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today!, Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// refactor to use closures -- ensures only one call to expensive calculator - this
// returns calling of expensive function severally -- how do we memoize
fn generate_workout_3(intensity: u32, random_number: u32) {
    let expensive_result = |num| {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result(intensity));
        println!("Next, do {} situps", expensive_result(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!, Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity));
        }
    }
}

// storing closure using generic parameters and Fn Traits -- to solve the prolem
// of calling the closure severally we memoize using a struct
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
where 
    T: Fn(u32) -> u32 {

        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
}

fn generate_workout_cached(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!, Remember to stay hydrated");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

// Cacher cannot be used with parameter types - while caching should support this: --
// how to do it? -- use hash map for specific keys and use generic parameters
