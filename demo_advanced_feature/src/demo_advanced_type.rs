// REF: https://rust-lang.tw/book-tw/ch19-04-advanced-types.html

pub fn demo_type_alias() {
    // The alias Kilometers is a synonym for i32.
    // Kilometers is not a separate, new type.
    // Values that have the type Kilometers will be treated the same as values of type i32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

pub fn demo_reduce_repetition() {
    // The main use case for type synonyms is to reduce repetition.
    type Thunk = Box<dyn Fn() + Send + 'static>; // ??? What is this.

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }
}

pub fn demo_never_type() {
    // TODO: review https://doc.rust-lang.org/book/ch19-04-advanced-types.html#the-never-type-that-never-returns
}

pub fn demo_dynamic_sized_type() {
    // TODO: review https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
}
