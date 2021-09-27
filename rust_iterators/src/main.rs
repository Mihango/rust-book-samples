#![allow(unused)]


fn main() {
   test_iter();
}

// iterator pattern allows to perform task on sequence of items in turn: note that 
// iterators are lazy and take effect on consuming them
fn test_iter() {
    let v1 = vec![1, 2, 3];

    let iter = v1.iter(); // does nothing

    // loop through the iteration
    for item in iter {
        println!("Got: {}", item);
    }
}

// using closures to capture thire environment : using filter iterator adaptor
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        )
    }
}

// creating own iterator with Iterator Trait
