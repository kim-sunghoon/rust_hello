#[macro_use]
extern crate fstrings;


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}


#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println_f!("{user1.email}");

    let user2 = build_user(String::from("someone@example.com"), String::from("someusername"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println_f!("{user2.username}");
    println_f!("{user3.username}");

    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println_f!("{user1:?}");
    println_f!("{rect1:#?}");

    println!(
        "The area of the rectangle with impl is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(3);

    println_f!("square area: {}", sq.area());

}
