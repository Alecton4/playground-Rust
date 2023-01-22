// REF: https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html
// REF: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html

fn main() {
    demo_mix();
    demo_while_let();
    demo_pattern_in_for_loop();
    demo_pattern_in_func_args();
    demo_multi_pattern();
    demo_match_range();
    demo_destructing_assignment();
    demo_destruct_struct();
    demo_destruct_enum();
    demo_destruct_nested();
    demo_match_guard();
    demo_match_guard_check_outer_vars();
    demo_at_binding();
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

fn demo_pattern_in_func_args() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // NOTE: We can also use patterns in closure parameter lists
    // in the same way as in function parameter lists,
    // because closures are similar to functions,
}

fn demo_multi_pattern() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn demo_match_range() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

fn demo_destructing_assignment() {
    // REF: https://course.rs/basic/variable.html#%E8%A7%A3%E6%9E%84%E5%BC%8F%E8%B5%8B%E5%80%BC
    struct Struct {
        e: i32,
    }

    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // "_" means match a value, but we do not care about the value. Thus we do not give it meaningful name.
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn demo_destruct_struct() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x: a, y: 0 } => println!("On the x axis at {a}"),
        Point { x: 0, y: b } => println!("On the y axis at {b}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

fn demo_destruct_enum() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            // For enum variants without any data, like Message::Quit,
            // we can’t destructure the value any further.
            // We can only match on the literal Message::Quit value,
            // and no variables are in that pattern.
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            // For struct-like enum variants, such as Message::Move,
            // we can use a pattern similar to the pattern we specify to match structs.
            // After the variant name, we place curly brackets and then list the fields with variables
            // so we break apart the pieces to use in the code for this arm.
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            // For tuple-like enum variants, like Message::Write that holds a tuple with one element
            // and Message::ChangeColor that holds a tuple with three elements,
            // the pattern is similar to the pattern we specify to match tuples.
            // The number of variables in the pattern must match the number of elements in the variant we’re matching.
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}

fn demo_destruct_nested() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}

fn demo_match_guard() {
    let num = Some(4);

    // A match guard is an additional if condition,
    // specified after the pattern in a match arm,
    // that must also match for that arm to be chosen.
    // NOTE: The downside of this additional expressiveness is that
    // the compiler doesn't try to check for exhaustiveness
    // when match guard expressions are involved.
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

fn demo_match_guard_check_outer_vars() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    // NOTE: The precedence of a match guard in relation to a pattern behaves like this:
    // (4 | 5 | 6) if y => ...
    // rather than this:
    // 4 | 5 | (6 if y) => ...
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

// NOTE: Using @ lets us test a value and save it in a variable within one pattern.
fn demo_at_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // By specifying id_variable @ before the range 3..=7,
            // we’re capturing whatever value matched the range while also testing that the value matched the range pattern.
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello {
            // In this second arm, where we only have a range specified in the pattern,
            // the code associated with the arm doesn’t have a variable that contains the actual value of the id field.
            // The id field’s value could have been 10, 11, or 12,
            // but the code that goes with that pattern doesn’t know which it is.
            // The pattern code isn’t able to use the value from the id field,
            // because we haven’t saved the id value in a variable.
            id: 10..=12,
        } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
