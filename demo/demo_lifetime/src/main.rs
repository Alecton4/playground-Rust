// REF: https://www.youtube.com/watch?v=juIINGuZyBc

use std::fmt::Display;

fn main() {
    demo_lifetime();
}

// NOTE: Generic lifetime annotation describes the relationship between the lifetimes of multiple references
// and how they relate to each other.
// It is a type of generics.
// Generic lifetime annotation does not change the actual lifetime of a reference
// but rather just explain the relationship between different lifetimes.

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// The example below says the lifetime of the returned reference will be the same as
// the smallest lifetime of the arguments.
// The lifetime of the return value always has to be tied to the lifetime of one the the arguments.
fn longer_str<'lifetime>(x: &'lifetime str, y: &'lifetime str) -> &'lifetime str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn demo_lifetime() {
    // let result;

    // let string1 = String::from("abc");
    // {
    //     let string2 = String::from("xyz");
    //     // The following statement is wrong because string2 does not live long enough.
    //     result = longer_str(string1.as_str(), string2.as_str());
    // }

    // println!("The longer one is {}", result);
}

// Struct that has lifetime annotations
// The example below says the struct cannot outlive the reference passed into "part".
struct ImportantExpert<'a> {
    part: &'a str,
}

// NOTE: Lifetime elision
// (Lifetime of arguments being passed in is called "input lifetime".)
// (Lifetime of returned values is called "output lifetime".)
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter,
//    that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters,
//    but one of them is "&self" or "&mut self",
//    the lifetime of self is assigned to all output lifetime parameters.

impl<'a> ImportantExpert<'a> {
    // The 3rd rule applies here.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

    fn announce_and_return_part_2<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        // NOTE: The following means 'a is not less than 'b.
        // Without this the program cannot compile
        // because "self.part" has 'a and "&'b str" has 'b.
        // The compiler needs to know the relationship between 'a and 'b.
        'a: 'b,
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// NOTE: Static lifetime means that the reference could live as long as the duration of the program.
// All the string literals have static lifetime.

// The following example combines generic, trait, and lifetime.
fn demo_generic_trait_lifetime<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
