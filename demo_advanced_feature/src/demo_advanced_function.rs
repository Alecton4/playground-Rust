// REF: https://rust-lang.tw/book-tw/ch19-05-advanced-functions-and-closures.html

pub fn demo_function_pointer() {
    // You can also pass regular functions to functions.
    // Functions coerce to the type fn (with a lowercase f),
    // not to be confused with the Fn closure trait.
    // The fn type is called a function pointer.
    // Passing functions with function pointers will allow you to use functions as arguments to other functions.

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    type T = fn(i32) -> i32;
    fn do_twice(f: T, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // NOTE: Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce),
    // meaning you can always pass a function pointer as an argument for a function that expects a closure.
    // It’s best to write functions using a generic type and one of the closure traits
    // so your functions can accept either functions or closures.

    // That said, one example of where you would want to only accept fn and not closures is
    // when interfacing with external code that doesn’t have closures:
    // C functions can accept functions as arguments, but C doesn’t have closures.
}

pub fn demo_returning_closure() {
    // Closures are represented by traits, which means you can’t return closures directly.
    // In most cases where you might want to return a trait,
    // you can instead use the concrete type that implements the trait as the return value of the function.
    // However, you can’t do that with closures because they don’t have a concrete type that is returnable;
    // you’re not allowed to use the function pointer fn as a return type.
    // The following wont compile:
    //
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    //
    // Rust doesn’t know how much space it will need to store the closure.
    // We can use a trait object to solve it.
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
