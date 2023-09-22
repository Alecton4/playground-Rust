use colored::Colorize;
use ferris_says::say;
use rand::Rng;

use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter, Write};

pub fn guess_hello() {
    let guess = get_input::<String>(
        "Please input your guess: ",
        "Cannot parse to String",
        "Your guess: ",
    );

    let message = if crate::contain_substring(&guess, "hello world") {
        String::from("Hello fellow Rustaceans!")
    } else {
        String::from("You are not a Rustacean!")
    };

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let width = message.chars().count();
    // !!! The BufWriter is buffering it and will only actually perform the write when it's dropped,
    // at the very end of the function.
    say(message.as_str(), width, &mut writer).unwrap();
    // Thus the following is printed first.
    println!("Good Bye!")
}

pub fn guess_number() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_num);

    loop {
        let guess_num: u32 = get_input(
            "Please guess the number: ",
            "Please enter a positive integer!",
            "Your guess: ",
        );

        match guess_num.cmp(&secret_num) {
            Ordering::Less => {
                println!("{}", "Too small!".red());
                continue;
            }
            Ordering::Greater => {
                println!("{}", "Too big".red());
                continue;
            }
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

// REF: https://dev.to/jahwi/a-simple-user-input-collection-validation-and-conversion-library-in-rust-34cj
fn get_input<T>(prompt: &str, parse_fail_prompt: &str, post_prompt: &str) -> T
where
    T: std::str::FromStr + std::fmt::Display,
{
    loop {
        print!("{}", prompt);
        stdout().flush().expect("Failed to flush stdout!");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line!");

        let parsed_input = match input.trim().parse::<T>() {
            Ok(value) => value,
            Err(_) => {
                println!("{}", parse_fail_prompt.red());
                continue;
            }
        };

        if !post_prompt.is_empty() {
            println!("{}{}", post_prompt, parsed_input);
        }

        return parsed_input;
    }
}
