use colored::Colorize;
use ferris_says::say;
use rand::Rng;
use std::{
    cmp::Ordering,
    io::{stdin, stdout, BufWriter},
};

fn main() {
    // greet_world();
    // guess_hello();
    guess_number();
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn guess_hello() {
    println!("Please input your guess.");

    let mut guess = String::new();
    stdin().read_line(&mut guess).expect("Failed to read line!");

    let message = if contain_substring(&guess, "hello world") {
        String::from("Hello fellow Rustaceans!")
    } else {
        String::from("You are not a Rustacean!")
    };

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let width = message.chars().count();

    say(message.as_bytes(), width, &mut writer).unwrap(); // !!! The BufWriter is buffering it and will only actually perform the write when it's dropped, at the very end of the function
    println!("Good Bye!") // Thus this is printed first
}

fn guess_number() {
    let secret_num = rand::thread_rng().gen_range(1..=2);
    println!("The secret number is {}", secret_num);

    loop {
        println!("Please input a number!");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line!");

        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess_num);

        match guess_num.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

// sort the array in descending order
fn sort_array(arr: &mut Vec<i32>) -> Vec<i32> {
    arr.sort_by(|a, b| b.cmp(a));
    arr.to_vec()
}

// search substrings in a string ignoring case
fn contain_substring(src: &str, sub: &str) -> bool {
    src.to_lowercase().contains(sub.to_lowercase().as_str())
}

// count substrings in a string ignoring case
fn count_substring(src: &str, sub: &str) -> i32 {
    src.to_lowercase()
        .matches(sub.to_lowercase().as_str())
        .count() as i32
}
