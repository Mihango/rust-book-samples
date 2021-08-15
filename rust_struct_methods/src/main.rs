#![allow(unused)]

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    rect.show_area();
    rect.show_area_pretty();
    let rect_2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "rect {:?} {} hold rect: {:?}",
        rect,
        if (rect.can_hold(&rect_2)) {
            "can"
        } else {
            "cannot"
        },
        rect_2
    );

    let square = Rectangle::square(34);
    square.show_area_pretty();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show_area_pretty(&self) {
        if self.width == self.height {
            println!(
                "Square dimension: {:?} and area >>> : {}",
                self,
                self.area()
            )
        } else {
            println!(
                "The area of rectangle:  {:#?} \nis {} sq pixels",
                self,
                self.area()
            );
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn show_area(&self) {
        println!("The area of rectangle is {} sq pixels", self.area());
    }

    // associated functions -> they are associated with the struct and dont take &self. Mostly used
    // for constructors that will return a new instance of the struct
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
