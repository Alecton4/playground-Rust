// REF: https://discord.com/channels/442252698964721669/448238009733742612/1065256784333394050
pub fn demo_destructing_assignment() {
    let mut val = (true, 2, "three");

    let tup = &mut val;
    // tup: &mut (bool, i32, &str)
    // simple enough, a mutable reference to a tuple

    let (a, b, c) = &mut val;
    // a: &mut bool
    // b: &mut i32
    // c: &mut &str
    // NOTE: &mut is "distributed" across destructured values

    *a = false;
    *b = 0;
    *c = "zero"; // makes c to point to "zero" instead of "three". cheap.

    // note that `a` doesn't need to be mutable.
    // you're not changing which data `a` points to,
    // you're changing the value of the data that `a` points to.
}

// REF: https://discord.com/channels/442252698964721669/448238009733742612/1065255249448808518
#[derive(Debug)]
enum List<'a> {
    Cons(i32, i32, i32, &'a List<'a>),
    Nil,
}

pub fn demo_pattern_matching() {
    let node_a = List::Cons(1, 1, 1, &List::Nil);
    let node_b = List::Cons(2, 2, 2, &List::Nil);
    let mut node_c = List::Cons(3, 3, 3, &node_a);

    println!("{:#?}", node_c);

    // The following is tedious.
    // if let List::Cons(value1, value2, value3, _) = node_c {
    //     node_c = List::Cons(value1, value2, value3, &node_b);
    // }

    // The following is wrong.
    // if let List::Cons(.., mut next) = node_c {
    //     next = &node_b;
    // }

    // What happens here is basically what happens in demo_destructing_assignment().
    if let List::Cons(.., next) = &mut node_c {
        *next = &node_b;
    }

    println!("{:#?}", node_c);
}
