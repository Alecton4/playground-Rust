// ref: https://www.youtube.com/watch?v=VFIOSWy93H0&list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8

// !!! ======== ownership rules ========
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of the scope, the value will be dropped
fn main() {}

fn demo_scope() {
    {
        // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward; memory is allocated on the heap
                                       // do stuff with s
    } // the scope is now over, and s is no longer valid; memory is automatically deallocated from the heap
}

fn demo_copy_and_move() {
    // Rust has a copy trait and basic types implements this trait
    let str1: &str = "hello";
    let str2 = str1; // copy
    println!("{}, world", str1);

    let x: i32 = 4;
    makes_copy(x);
    println!("{}", x);

    // String does not implement copy trait
    let string1: String = String::from("hello");
    let string2 = string1; // !!! move (NOT shadow copy)
    let string3 = string2.clone(); // this is copy
    println!("{}, world", string2);

    takes_ownership(string3);
    // println!("{}", string3); // this cannot be done

    let s1 = gives_ownership();
    let s2 = takes_and_gives_back(s1);
    println!("{}", s2);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

// !!! ======== reference rules ========
// 1. reference is immutable by default.
// 2. You can have EITHER any number of immutable references OR one mutable reference
//    (to a particular data in a particular scope at any given time ),
// 3. References must always be valid

// We do not want to take the ownership of a string.
// So we first take it then return it back.
// But this is tedious.
fn calculate_length_without_ref(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

// We do not want to take the ownership of a string.
// So we use reference.
fn calculate_length_with_ref(some_string: &String) -> usize {
    let length = some_string.len();
    length
}

fn change() {
    let mut some_string = String::from("hello");

    // You can have multiple immutable references
    let ref1 = &some_string;
    let ref2 = &some_string;
    println!("{} {}", ref1, ref2);

    // ref1 and ref2 are out of scope after println!
    let ref3 = &mut some_string;
    println!("{}", ref3);
}

// reference that points to nothing
// fn dangle() -> &String {
//     let some_string = String::from("hello");
//     &some_string
// }

// return the index to the end of the first word
fn first_word_without_slice(some_string: &String) -> usize {
    let bytes = some_string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    some_string.len()
}

// slices let you reference a contiguous sequence of elements within a collection instead of referencing the entire collection
fn first_word_with_slice(some_string: &String) -> &str {
    let bytes = some_string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }

    // let slice1 = some_string[0..2]; // !!! error
    // let slice2 = &some_string[0..2];

    // let a = [1, 2, 3, 4, 5];
    // let slice1 = a[0..2]; // !!! error
    // let slice2 = &a[0..2];

    &some_string[..]
}
