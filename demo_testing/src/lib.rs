// REF: https://www.youtube.com/watch?v=18-7NoNPO30
// TODO: https://www.youtube.com/watch?v=-L4nKAlmH3M

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn just_panic() {
    panic!("This function panics!");
}

#[cfg(test)]
mod tests {
    // NOTE: The product code is in the default module.
    // The tests are in the test module.
    // Thus we have to bring product code into scope.
    use super::*;

    #[test]
    fn area_is_correct() {
        let larger = Rectangle {
            width: 8,
            height: 9,
        };
        let smaller = Rectangle {
            width: 6,
            height: 4,
        };

        // NOTE: Both parameters have to implement the "PartialEq" and "Debug" traits.
        assert_eq!(72, larger.area());
        assert_ne!(larger.area(), smaller.area());
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 9,
        };
        let smaller = Rectangle {
            width: 6,
            height: 4,
        };

        assert!(larger.can_hold(&smaller), "Wrong implement of can_hold!");
    }

    #[test]
    #[should_panic(expected = "This function panics!")]
    fn test_panic() {
        just_panic();
    }

    // ??? The use case.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("Wrong adding!"))
        }
    }
}
