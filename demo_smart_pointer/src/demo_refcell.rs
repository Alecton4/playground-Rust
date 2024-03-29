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

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)); // This is interior mutability.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// NOTE: Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
// TODO: review
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

pub fn demo_combine_rc_and_refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = List::Cons(Rc::clone(&value), Rc::new(List::Nil));
    let a_rc = Rc::new(a);

    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a_rc));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a_rc));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a_rc);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List2 {
    Cons(Rc<RefCell<i32>>, RefCell<Rc<List2>>),
    Nil,
}

pub fn demo_combine_rc_and_refcell_2() {
    let value = Rc::new(RefCell::new(5));

    let a = List2::Cons(Rc::clone(&value), RefCell::new(Rc::new(List2::Nil)));
    let a = Rc::new(a);
    let b = List2::Cons(Rc::new(RefCell::new(3)), RefCell::new(Rc::clone(&a)));
    let b = Rc::new(b);
    let c = List2::Cons(Rc::new(RefCell::new(4)), RefCell::new(Rc::clone(&a)));
    let c = Rc::new(c);
    println!("c init = {:?}", c);

    *value.borrow_mut() += 10;

    if let List2::Cons(.., next) = &*c {
        // REF: https://doc.rust-lang.org/reference/expressions.html#expression-precedence
        let mut temp = next.borrow_mut(); // Same as (*next).borrow_mut()
        println!("c's next = {:?}", temp);
        *temp = Rc::clone(&b);
    }

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
