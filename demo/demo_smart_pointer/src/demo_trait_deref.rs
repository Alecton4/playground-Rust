// REF: https://doc.rust-lang.org/book/ch15-02-deref.html
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn demo_deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref())); // Actually code.
}

// Rust does deref coercion when it finds types and trait implementations in three cases:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>
pub fn demo_implicit_deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]); // Actually code.
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
