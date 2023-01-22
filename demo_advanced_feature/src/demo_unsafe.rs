// REF: https://rust-lang.tw/book-tw/ch19-01-unsafe-rust.html

// Unsafe Superpowers
// - Dereference a raw pointer
// - Call an unsafe function or method
// - Access or modify a mutable static variable
// - Implement an unsafe trait
// - Access fields of unions

// NOTE: unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks:
// if you use a reference in unsafe code, it will still be checked.
// The unsafe keyword only gives you access to these five features that
// are then not checked by the compiler for memory safety.
// You’ll still get some degree of safety inside of an unsafe block.

pub mod demo_raw_pointer {

    // Unsafe Rust has two new types called raw pointers that are similar to references.
    // As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
    // The asterisk isn’t the dereference operator; it’s part of the type name.
    // In the context of raw pointers,
    // immutable means that the pointer can’t be directly assigned to after being dereferenced.

    // Different from references and smart pointers, raw pointers:
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // - Aren’t guaranteed to point to valid memory
    // - Are allowed to be null
    // - Don’t implement any automatic cleanup
    pub fn demo_raw_pointer_from_ref() {
        let mut num = 5;

        // NOTE: We can create raw pointers in safe code;
        // we just can’t dereference raw pointers outside an unsafe block.
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
    }

    pub fn demo_raw_pointer_to_arbitrary_mem() {
        use std::slice;

        let address = 0x01234usize;
        let r = address as *mut i32;

        // NOTE: We don’t own the memory at this arbitrary location,
        // and there is no guarantee that the slice this code creates contains valid i32 values.
        // Attempting to use values as though it’s a valid slice results in undefined behavior.
        let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    }

    pub fn demo_deref_raw_pointer() {
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }
}

pub mod demo_unsafe_func_method {

    pub fn demo_unsafe_call() {
        unsafe fn dangerous() {}

        // We must call the dangerous function within a separate unsafe block.
        unsafe {
            dangerous();
        }
    }

    pub fn demo_safe_abstraction_over_unsafe() {
        // NOTE: The following code cannot compile because we have two mutable refs at the same time
        // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        //     let len = values.len();

        //     assert!(mid <= len);

        //     (&mut values[..mid], &mut values[mid..])
        // }

        use std::slice;

        // We don’t need to mark the resulting split_at_mut function as unsafe,
        // and we can call this function from safe Rust.
        // We’ve created a safe abstraction to the unsafe code
        // with an implementation of the function that uses unsafe code in a safe way,
        // because it creates only valid pointers from the data this function has access to.
        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }

        let mut vector = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut(&mut vector, 3);
    }
}

pub mod demo_call_external_code {
    // TODO: review https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
}

pub mod demo_static_vars {

    // In Rust, global variables are called static variables.
    pub static HELLO_WORLD: &str = "Hello, world!";

    // A subtle difference between constants and immutable static variables is that
    // values in a static variable have a fixed address in memory.
    // Using the value will always access the same data.
    // Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
    // Another difference is that static variables can be mutable.
    // Accessing and modifying mutable static variables is unsafe.
    static mut COUNTER: u32 = 0;

    pub fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    pub fn print_count() {
        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}

pub mod demo_unsafe_trait {

    // TODO: review https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait
    // If we implement a type that contains a type that is not Send or Sync,
    // such as raw pointers,
    // and we want to mark that type as Send or Sync, we must use unsafe.
    // Rust can’t verify that our type upholds the guarantees that
    // it can be safely sent across threads or accessed from multiple threads;
    // therefore, we need to do those checks manually and indicate as such with unsafe.
}

pub mod demo_union {

    // TODO: review https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-fields-of-a-union
}
