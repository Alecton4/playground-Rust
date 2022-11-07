struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // !!! cannot make one particular field mutable, must make the entire struct mutable
    let mut user1 = User {
        username: String::from("haha"),
        email: String::from("haha@haha.com"),
        sign_in_count: 0,
        active: false,
    };

    let name = user1.username;
    user1.email = String::from("haha@ust.hk");

    let user2 = build_user(String::from("hehe"), String::from("hehe@ust.hk"));

    let user3 = User {
        username: String::from("hoho"),
        email: String::from("hoho.ust.hk"),
        ..user2 // the remaining fields are the same as user2
    };

    // tuple structs (structs without name fields)
    // Useful when you want your entire tuple to have a name and be of different type than other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 89,
        hight: 64
    };
    rect.area(); // !!! automatic referencing and dereferencing

    let rect2 = Rectangle::square(64);
    rect2.area();
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // field init shorthand syntax
        email,    // field init shorthand syntax
        sign_in_count: 0,
        active: false,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
}

impl Rectangle {
    // the first argument of a method is always self
    fn area(&self) -> u32 {
        println!("rect: {:#?}", self);
        self.width * self.hight
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.hight > other.hight
    }
}

impl Rectangle {
    // we can also defined associated function (not method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            hight: size,
        }
    }
}
