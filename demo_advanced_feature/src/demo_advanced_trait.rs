// REF: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
pub fn demo_associated_type() {
    pub trait Iterator {
        // The type Item is a placeholder,
        // and the next method’s definition shows that it will return values of type Option<Self::Item>.
        // Implementors of the Iterator trait will specify the concrete type for Item,
        // and the next method will return an Option containing a value of that concrete type.
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            // --snip--
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // TODO: review https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types
    // pub trait Iterator<T> {
    //     fn next(&mut self) -> Option<T>;
    // }
}

pub fn demo_operator_overloading() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // NOTE: The default generic type in this code is within the Add trait. Here is its definition:
    //
    // trait Add<Rhs = Self> {
    //     type Output;
    //
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    //
    // The Rhs generic type parameter (short for “right hand side”)
    // defines the type of the rhs parameter in the add method.
    // If we don’t specify a concrete type for Rhs when we implement the Add trait,
    // the type of Rhs will default to Self, which will be the type we’re implementing Add on.
}

pub fn demo_default_type_parameter() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Millimeters(u32);

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Meters(u32);

    // To add Millimeters and Meters,
    // we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000))
}

pub fn demo_specify_trait_before_method_call() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

pub fn demo_fully_qualified_syntax() {
    // Associated functions that are not methods don’t have a self parameter.
    // When there are multiple types or traits that define non-method functions with the same function name,
    // Rust doesn't always know which type you mean unless you use fully qualified syntax.
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

pub fn demo_supertrait() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    Point { x: 89, y: 64 }.outline_print();
}

pub fn demo_new_type_pattern() {
    // TODO: review https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
