// REF: https://doc.rust-lang.org/book/ch16-02-message-passing.html
// TODO: review

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn demo_mpsc_basics() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // NOTE: The send method returns a Result<T, E> type,
        // so if the receiver has already been dropped and there’s nowhere to send a value,
        // the send operation will return an error.
        // In this example, we’re calling unwrap to panic in case of an error.
        tx.send(val).unwrap();
    });

    // NOTE: The receiver has two useful methods: recv and try_recv.
    // We’re using recv, short for receive,
    // which will block the main thread’s execution and wait until a value is sent down the channel.
    // The try_recv method doesn’t block,
    // but will instead return a Result<T, E> immediately:
    // an Ok value holding a message if one is available and
    // an Err value if there aren’t any messages this time.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn demo_receiver_waiting() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // NOTE: We’re not calling the recv function explicitly anymore:
    // instead, we’re treating rx as an iterator.
    // For each value received, we’re printing it.
    // When the channel is closed, iteration will end.
    // Because we don’t have any code that pauses or delays in the for loop in the main thread,
    // we can tell that the main thread is waiting to receive values from the spawned thread.
    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn demo_multi_producer() {
    let (tx0, rx) = mpsc::channel();
    let tx1 = tx0.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx0.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    let handle_receiver = thread::spawn(move || {
        for received in rx {
            println!("Got: {}", received);
        }
    });

    handle_receiver.join().unwrap();
}
