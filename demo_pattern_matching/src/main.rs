fn main() {
    demo_mix();
    demo_while_let();
    demo_pattern_in_for_loop();
    demo_destructing_assignment();
}

// Mixing if let, else if, else if let, and else
fn demo_mix() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // NOTE: The line if let Ok(age) = age introduces a new shadowed age variable that
        // contains the value inside the Ok variant.
        // This means we need to place the if age > 30 condition within that block:
        // we can’t combine these two conditions into if let Ok(age) = age && age > 30.
        // The shadowed age we want to compare to 30 isn’t valid until the new scope starts with the curly bracket.
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn demo_while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn demo_pattern_in_for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// REF: https://course.rs/basic/variable.html#%E8%A7%A3%E6%9E%84%E5%BC%8F%E8%B5%8B%E5%80%BC
struct Struct {
    e: i32,
}

fn demo_destructing_assignment() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // "_" means match a value, but we do not care about the value. Thus we do not give it meaningful name.
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
