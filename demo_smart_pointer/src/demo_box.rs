// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.
pub fn demo_box_basics() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs:
// a pointer’s size doesn’t change based on the amount of data it’s pointing to.
// This means we can put a Box<T> inside the Cons variant instead of another List value directly.
// The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::demo_box::List::{Cons, Nil}; // NOTE: review

pub fn demo_list_using_box() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", list);
}
