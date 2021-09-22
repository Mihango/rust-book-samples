#![allow(unused)]

#[cfg(test)]
mod tests {
    #[test]
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_workers() {
        let y = 3;
        assert_eq!(3, y);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail")
    // }
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // usig assert equals - can use any value of left or right parameters
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // testing with contains
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
    // String::from("Hello!")
}

// testing should panic macro
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be less than or equal to 100");
        }
        Guess { value }
    }
}

mod test_guess {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(109);
    }
}

// testing and showing outputs
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

mod test_outputs {
    use super::*;

    #[test]
    fn passing_test() {
        let value = prints_and_returns_10(34);
        assert_eq!(10, value);
    }
}
