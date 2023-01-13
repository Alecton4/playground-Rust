// REF: https://doc.rust-lang.org/book/ch15-03-drop.html
// The second trait important to the smart pointer pattern is Drop,
// which lets you customize what happens when a value is about to go out of scope.
// You can provide an implementation for the Drop trait on any type,
// and that code can be used to release resources like files or network connections.

struct CustomSmartPointer {
    data: String,
}

// The Drop trait is included in the prelude, so we don’t need to bring it into scope.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn demo_drop_basics() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

pub fn demo_early_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // NOTE: The std::mem::drop function is different from the drop method in the Drop trait.
    // We call it by passing as an argument the value we want to force drop.
    // The function is in the prelude.
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
