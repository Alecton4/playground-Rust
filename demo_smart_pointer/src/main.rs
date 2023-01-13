// REF: https://doc.rust-lang.org/book/ch15-01-box.html
mod demo_reference_counting;
mod demo_trait_deref;
mod demo_trait_drop;

fn main() {
    demo_box_basics();
    demo_list_using_box();
    demo_trait_deref::demo_deref();
    demo_trait_deref::demo_implicit_deref_coercion();
    demo_trait_drop::demo_drop_basics();
    demo_trait_drop::demo_early_drop();
    demo_reference_counting::demo_rc();
    demo_reference_counting::demo_rc_2();
}

// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.
fn demo_box_basics() {
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

use crate::List::{Cons, Nil}; // NOTE: review

fn demo_list_using_box() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:#?}", list);
}
