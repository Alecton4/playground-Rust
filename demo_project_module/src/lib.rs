// ref: https://www.youtube.com/watch?v=5RPXgDQrjio

mod front_of_house; // the content of the module is place in a file with the same name

mod back_of_house {
    pub enum Appetizer {
        // if an enum is public, its variants are also made public
        Soup,
        Salad,
    }

    pub struct Breakfast {
        // the public field of a strut has to be explicitly public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting; // absolute path
                                    // use self::front_of_house::hosting; //relative path
                                    // use front_of_house::hosting; // relative path

// re-export module
pub use rand::{CryptoRng, Rng};
pub use std::io::{self, Write};

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path
    front_of_house::hosting::add_to_waitlist();
    // after using the "use" keyword
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}
