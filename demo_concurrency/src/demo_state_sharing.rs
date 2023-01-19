// REF: https://doc.rust-lang.org/book/ch16-03-shared-state.html
// NOTE: review

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn demo_mutex_basics() {
    let m = Mutex::new(5);

    {
        // The type system ensures that we acquire a lock before using the value in m.
        // The type of m is Mutex<i32>, not i32, so we must call lock to be able to use the i32 value.
        let mut num = m.lock().unwrap();
        // ??? Mutex<T> is a smart pointer.
        // More accurately, the call to lock returns a smart pointer called MutexGuard,
        // wrapped in a LockResult that we handled with the call to unwrap.
        // The MutexGuard smart pointer implements Deref to point at our inner data;
        // the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope,
        // which happens at the end of the inner scope.
        *num = 6;
    }

    println!("m = {:?}", m);
}

pub fn demo_multi_thread_error_1() {
    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         // NOTE: We can’t move the ownership of lock counter into multiple threads.
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
}

pub fn demo_multi_thread_error_2() {
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         // NOTE: Rc<T> is not safe to share across threads.
    //         // When Rc<T> manages the reference count,
    //         // it adds to the count for each call to clone and subtracts from the count when each clone is dropped.
    //         // But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
}

pub fn demo_multi_thread_with_Arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// REF: https://doc.rust-lang.org/book/ch16-03-shared-state.html#similarities-between-refcelltrct-and-mutextarct
// REF: https://doc.rust-lang.org/std/sync/struct.Mutex.html
pub fn demo_deadlock() {
    // TODO
}
