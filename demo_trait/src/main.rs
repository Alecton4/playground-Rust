// REF: https://www.youtube.com/watch?v=T0Xfltu4h3A

use std::fmt::{Debug, Display};

fn main() {
    let article = NewsArticle {
        author: String::from("John"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling."),
    };

    let tweet = Tweet {
        username: String::from("john"),
        content: String::from("hello, world"),
        reply: false,
        retweet: false,
    };

    println!("Article sum: {}", article.summarize());
    println!("Tweet sum: {}", tweet.summarize());

    demo_impl_syntax(&article);

    println!("{}", demo_return_impl().summarize());
}

pub trait Summary {
    // NOTE: Must be implemented
    fn summarize_author(&self) -> String;

    // NOTE: Default implement
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }

    // fn summarize(&self) -> String {
    //     format!("\"{}\", by {}", self.headline, self.author)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// Traits as parameters.
pub fn demo_impl_syntax(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// NOTE: Above is the syntax sugar for the following trait bound syntax:
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn demo_trait_bound_syntax<T: Summary + Display>(item1: &T, item2: &T) {
    // Do something...
}

fn demo_where_syntax<T, U>(item1: &T, item2: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    8964
}

// Traits as return type.
// Return any type that implements Summary trait.
fn demo_return_impl() -> impl Summary {
    Tweet {
        username: String::from("john"),
        content: String::from("LOL"),
        reply: false,
        retweet: false,
    }
}

// !!! You can only return one specific type
// The following code is not allowed.
// fn demo_return_impl_2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             author: String::from("John"),
//             headline: String::from("A Headline"),
//             content: String::from("Some contents"),
//         }
//     } else {
//         Tweet {
//             username: String::from("john"),
//             content: String::from("LOL"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Use trait to conditionally implement methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // NOTE: Use "Self" instead of "self"
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T>
where
    T: Display + PartialOrd,
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger member is x");
        } else {
            println!("The larger member is y");
        }
    }
}

// ???
// Implement a trait on oa type that implements another trait
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }
