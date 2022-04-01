#![allow(unused)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Associated function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("mic.a.elle.chlon@gmail.com"),
        username: String::from("Michaël"),
        active: true,
        sign_in_count: 1,
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let name = user1.username;
    user1.username = String::from("MrMic");

    let user2 = build_user(String::from("m.chlon@wanadoo.fr"), String::from("Mimine"));
    let user3 = User {
        email: String::from("john@gmail.com"),
        username: String::from("John"),
        ..user2
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect4 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect2));
    println!("rect can hold rect2: {}", rect2.can_hold(&rect3));

    println!("rect: {:#?}", rect);

    println!("The area of the rectangle is {} square pixels", rect.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
