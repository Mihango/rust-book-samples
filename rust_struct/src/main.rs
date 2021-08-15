#![allow(unused)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // creating a struct instance
    let user = User {
        email: String::from("someoned@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // creating a struct instance using method
    let user_2 = build_user(String::from("sample@mail.com"), String::from("userheer"));

    // creating a struct from another struct
    let user_3 = User {
        email: String::from("user3@mail.com"),
        username: String::from("user3"),
        ..user_2
    };

    println!("user email: {}", user.email);
    println!("user 3 details: {}, {}, {}", user_3.email, user_3.active, user_3.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
