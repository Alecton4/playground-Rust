// REF: https://www.youtube.com/watch?v=XYkiwsplDTg

use std::env; // To take user args.
use std::error::Error;
use std::fs; // To read files.
use std::process; //To exit program without panic.

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Use of closure
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file: {}", config.filename);

    // NOTE: In this case we only care about the error variant,
    // so we can use the if let syntax.
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        // NOTE: Panic is more appropriate for programming errors rather than usage errors
        // if args.len() < 3 {
        //     panic!("Not enough arguments!");
        // }

        if args.len() < 3 {
            Err("Not enough arguments!")
        } else {
            Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
            })
        }
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // NOTE: expect() will panic.
    // let contents = fs::read_to_string(config.filename).expect("Cannot read file into string!");

    // NOTE: "?" here mean if read_to_string() returns an Error type,
    // then this Error type is automatically be returned from this run() function.
    let contents = fs::read_to_string(config.filename)?;
    Ok(())
}
