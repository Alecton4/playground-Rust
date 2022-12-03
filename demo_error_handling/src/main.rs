// REF: https://www.youtube.com/watch?v=wM6o70NAWUI

use std::fs::{self, File};
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // demo_panic_macro();
    // demo_recoverable_errors();
    demo_unwrap();
}

fn demo_panic_macro() {
    panic!("crash and burn!");
}

fn demo_recoverable_errors() {
    let f = File::open("file.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(file_created) => file_created,
                Err(e) => panic!("Cannot create file: {:?}", e),
            },
            any_other_error => {
                panic!("Cannot open: {:?}", any_other_error)
            }
        },
    };
}

fn demo_recoverable_errors_2() {
    let f = File::open("file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("file.txt").unwrap_or_else(|error| {
                panic!("Cannot create file: {:?}", error);
            })
        } else {
            panic!("Cannot open: {:?}", error);
        }
    });
}

fn demo_unwrap() {
    let f = File::open("file.txt").unwrap();
    // This is doing the same thing as below
    // let f = File::open("file.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!(),
    // };
}

fn demo_expect() {
    let f = File::open("file.txt").expect("Cannot open file");
}

fn demo_error_propagation() -> Result<String, Error> {
    let f = File::open("file.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn demo_error_propagation_2() -> Result<String, Error> {
    let mut f = File::open("file.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn demo_error_propagation_3() -> Result<String, Error> {
    let mut s = String::new();
    File::open("file.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn demo_error_propagation_4() -> Result<String, Error> {
    fs::read_to_string("file.txt")
}
