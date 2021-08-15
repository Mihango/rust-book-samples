#![allow(unused)]

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let home = IpAddress {
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: six,
        address: String::from("::1"),
    };

    loopback.pretty_print();
    home.pretty_print();

    let ip_address = IpAddressKindWithValue::V4(String::from("127.0.0.1"));
    ip_address.pretty_print();

    // using message types
    let quit = Message::QUIT;
    quit.call();

    // using option enum
    let some_number = Some(5);
    let some_string = Some(String::from("Hello"));
    let absent: Option<i32> = None;
}

// enum with ip address type
#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

// use the enum in a struct and create a ip address
#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

impl IpAddress {
    fn pretty_print(&self) {
        println!("Ip Address : {:?}", self);
    }
}

// above enum and struct can be reperesented in a single enum
#[derive(Debug)]
enum IpAddressKindWithValue {
    V4(String),
    V6(String),
}

impl IpAddressKindWithValue {
    fn pretty_print(&self) {
        println!("Ip Address : {:?}", self);
    }
}

// enums can have different values
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum with different variants
#[derive(Debug)]
enum Message {
    QUIT,
    MOVE { x: i32, y: i32 },
    WRITE(String),
    CHANGECOLOR(i32, i32, i32),
}

// enums also have method
impl Message {
    fn call(&self) {
        match self {
            Message::QUIT => println!("Quiting"),
            Message::MOVE {x, y} => println!("Moving direction {:?}, {}, {}", self, x, y),
            Message::WRITE(message) => println!("Writing >>> {:?} >>> {}", self, message),
            Message::CHANGECOLOR(r, g, b) => println!("Changing color >>>> {:?}", self),
        }
    }
}
