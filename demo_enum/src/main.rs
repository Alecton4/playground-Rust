// ref: https://www.youtube.com/watch?v=DSZqIJhkNCM&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // NOTE: anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can define methods and associated functions on the enum type
impl Message {
    fn some_function() {
        println!("hello world");
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6;
}

fn demo_Option() {
    let some_num = Some(8964);
    let some_str = Some("rust");
    // !!! type annotations needed because None is used
    let absent_num: Option<i32> = None;
}

fn demo_unwrap() {
    let x: i8 = 89;
    let y = Some(64);
    let z = None;
    let sum = x + y.unwrap_or(0) + z.unwrap_or(0);
}

fn demo_match(some_int: Option<i32>) -> Option<i32> {
    match some_int {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn demo_if_let() {
    let some_value = Some(3);

    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_value {
        println!("three")
    }
}
