// REF: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
// NOTE: Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
// 1. Rc<T> enables multiple owners of the same data;
//    Box<T> and RefCell<T> have single owners.
// 2. Box<T> allows immutable or mutable borrows checked at compile time;
//    Rc<T> allows only immutable borrows checked at compile time;
//    RefCell<T> allows immutable or mutable borrows checked at runtime.
// 3. Because RefCell<T> allows mutable borrows checked at runtime,
//    you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
// NOTE: Mutating the value inside an immutable value is the interior mutability pattern.
