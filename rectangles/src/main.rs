#![allow(unused)]
fn main() {
    // area calculation using variables
    let width = 30;
    let height = 50;

    println!(
        "The are of the rectanlge is {} square pixels",
        area(width, height)
    );

    // area calculations using tuple
    let area = area_tuple((30, 40));
    println!("The are of the rectanlge is {} square pixels", area);

    // area calculations using struct
    let rect = Rectangle {
        width: 20,
        height: 40,
    };
    println!("rect is {:#?}", rect);
    let area_2 = area_struct(&rect);
    println!("The are of the rectanlge is {} square pixels", area_2);
}

// area calculator with variables
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// area calculator with tuples
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// area calculator using struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
