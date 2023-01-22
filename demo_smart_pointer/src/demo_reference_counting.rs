// REF: https://doc.rust-lang.org/book/ch15-04-rc.html
// NOTE: The following is not doable:
// use crate::List::{Cons, Nil};
// fn demo_rc() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

// The following is doable
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

pub fn demo_rc() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));
    println!("{:#?}", b);
    println!("{:#?}", c);
}

pub fn demo_rc_2() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
