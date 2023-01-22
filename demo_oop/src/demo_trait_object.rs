// REF: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
// TODO: review trait object

use demo_oop::lib_demo_trait_object::{Button, Draw, Screen};

// User can define his own struct implementing Draw.
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Draw SelectBox.
    }
}

pub fn demo_trait_object() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 89,
                height: 64,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 89,
                height: 64,
                label: String::from("ok"),
            }),
        ],
    };

    screen.run();
}

// TODO: review https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
// TODO: review https://doc.rust-lang.org/reference/items/traits.html#object-safety
