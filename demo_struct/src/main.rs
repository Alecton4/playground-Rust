struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // !!! Cannot make one particular field mutable, must make the entire struct mutable
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
        ..user2 // The remaining fields are the same as user2
    };

    // Tuple structs (structs without name fields)
    // Useful when you want your entire tuple to have a name and be of different type than other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        width: 89,
        height: 64,
    };
    // NOTE: Automatic referencing and dereferencing:
    // Calling a method on an instance directly is the same as calling it on a pointer to the instance.
    rect.area();

    let rect2 = Rectangle::square(64);
    rect2.area();
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // Field init shorthand syntax
        email,    // Field init shorthand syntax
        sign_in_count: 0,
        active: false,
    }
}

// Makes the compiler to provide a basic implementation of the Debug trait.
// Allow us to print out information useful to developers.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // The first argument of a method is always "self".
    fn area(&self) -> u32 {
        println!("rect: {:#?}", self);
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // We can also defined associated function (not method).
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
