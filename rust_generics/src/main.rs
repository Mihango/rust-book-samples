#![allow(unused)]

// Generic are one of tools in any programming language for handling duplication of concepts

fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    let largest = largest_value(&list);

    let largest_2 = largest_value(&chars);

    println!("The largest number in {:?} is {}", list, largest);
    println!("The largest number in {:?} is {}", chars, largest_2);
}

// use below function to detect duplication
fn find_largest_num(data: &Vec<i32>) -> i32 {
    let mut largest = data[0];

    for &num in data {
        if(num > largest) {
            largest = num; 
        }
    }

    largest
}

fn find_largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic function
fn largest_value<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
