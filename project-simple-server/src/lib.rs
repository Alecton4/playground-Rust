// REF: https://doc.rust-lang.org/book/ch20-02-multithreaded.html
// REF: https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    // TODO
    // pub fn build(size: usize) -> Result<ThreadPool, &'static str> {
    //     match size {
    //         0 => Err("Pool size cannot be 0!"),
    //         _ => Ok(ThreadPool),
    //     }
    // }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // NOTE: review
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}...", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing...");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down...");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// TODO: review why this does not work as expected
// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
//         let thread = thread::spawn(move || {
//             while let Ok(job) = receiver.lock().unwrap().recv() {
//                 println!("Worker {id} got a job; executing.");

//                 job();
//             }
//         });

//         Worker { id, thread }
//     }
// }

type Job = Box<dyn FnOnce() + Send + 'static>;
