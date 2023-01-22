// REF: https://www.youtube.com/watch?v=4GcKrj4By8k
// REF: https://doc.rust-lang.org/book/ch13-02-iterators.html

fn main() {
    demo_vector_iter();
}

fn demo_vector_iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for value in v1_iter {
        println!("Got: {}", value);
    }
}

#[test]
fn demo_iter_next() {
    let v1 = vec![1, 2, 3];

    // Get reference.
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // Get owned type.
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
// Methods that call next are called consuming adaptors, because calling them uses up the iterator.
// One example is the sum method,
// which takes ownership of the iterator and iterates through the items by repeatedly calling next,
// thus consuming the iterator.
fn demo_consuming_adaptor() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    assert_eq!(v1_iter.sum::<i32>(), 6);
}

#[test]
// Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
fn demo_adaptor() {
    let v1 = vec![1, 2, 3];
    // map() takes a closure to call on each item as the items are iterated through.
    // The map method returns a new iterator that produces the modified items.
    // The closure here creates a new iterator in which each item from the vector will be incremented by 1:
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoe_list: Vec<Shoe>, my_size: u32) -> Vec<Shoe> {
    shoe_list
        .into_iter()
        .filter(|s| s.size == my_size) // my_size is not defined within the closure but we can still access it. (cap env)
        .collect()
}

#[cfg(test)]
mod demo_capture_environment {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 39,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 42,
                style: String::from("sandal"),
            },
            Shoe {
                size: 43,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 43);
        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 43,
                style: String::from("boot"),
            }]
        );
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // NOTE: Associated type is used here.
    // Here it means that our iterator is going to return items of type u32.
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn demo_our_own_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    assert_eq!(
        18,
        Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum::<u32>()
    );
}
