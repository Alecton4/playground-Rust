// REF: https://doc.rust-lang.org/book/ch16-01-threads.html

use std::thread;
use std::time::Duration;

pub fn demo_spawn() {
    // NOTE: When the main thread of a Rust program completes,
    // all spawned threads are shut down, whether or not they have finished running.
    thread::spawn(|| {
        for i in 0..100 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..50 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(4));
    }
}

pub fn demo_join() {
    let handle = thread::spawn(|| {
        for i in 0..100 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..50 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // NOTE: Calling join on the handle blocks the thread currently running
    // until the thread represented by the handle terminates.
    handle.join().unwrap();
}

pub fn demo_join_2() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Small details, such as where join is called,
    // can affect whether or not your threads run at the same time.
    handle.join().unwrap();

    for i in 0..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn demo_move_error() {
    // let v = vec![1, 2, 3];

    // // Rust can’t tell how long the spawned thread will run,
    // // so it doesn’t know if the reference to v will always be valid.
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // // Potentially being invalid.
    // drop(v); // oh no!

    // handle.join().unwrap();
}

pub fn demo_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
