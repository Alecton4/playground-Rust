// REF: https://www.youtube.com/watch?v=DSZqIJhkNCM
// TODO: Review

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

    demo_Option();
    demo_unwrap();
    demo_match(Some(8964));
    demo_if_let();
}

fn demo_Option() {
    let some_num = Some(8964);
    let some_str = Some("rust");
    // !!! Type annotations needed because None is used.
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
    let a = 8964;
    let some_value = Some(a);

    match some_value {
        Some(8964) => println!("three"),
        _ => (),
    }

    // NOTE: Useful when we only care about one variant
    if let Some(8964) = some_value {
        println!("three")
    }
}
