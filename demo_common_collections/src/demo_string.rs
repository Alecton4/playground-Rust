use unicode_segmentation::UnicodeSegmentation;

pub fn demo_string_basics() {
    // REF: https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
    // REF: https://stackoverflow.com/questions/49393462/what-does-str-does-not-have-a-constant-size-known-at-compile-time-mean-and
    // Strings are stored as a collection of UTF-8 encoded bytes
    let s1 = String::new();
    let s2 = "initial contents"; // define a string slice
    let s3 = s2.to_string(); // turn a string slice into an own String
    let s4 = String::from("initial contents"); // create an own String from a string slice
}

pub fn demo_string_append() {
    let mut s = String::from("foo");
    // append str using push_str
    s.push_str("bar");
    // append char using push
    s.push('!');

    let s1 = String::from("hello, ");
    let s2 = String::from("world");

    // concatenate using format
    let s3 = format!("{}{}", s1, s2); // NOTE: format! does not take the ownership of s1 and s2

    // concatenate using +
    let s4 = s1 + &s2; // NOTE: take the ownership of s1 and concatenate
}

pub fn demo_string_index() {
    let hello = String::from("hello");
    // This is not doable because some characters might be more than one byte
    // let c = hello[0];

    // byte
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // scaler value
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // grapheme value
    for g in "नमस्ते".graphemes(true) {
        println!("{}", g);
    }
}
