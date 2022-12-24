// REF: https://www.youtube.com/watch?v=XYkiwsplDTg
// REF: https://www.youtube.com/watch?v=AABHxixn6Cw

use std::env; // To take user args.
use std::process; //To exit program without panic.

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Use of closure
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file: {}", config.filename);

    // NOTE: In this case we only care about the error variant,
    // so we can use the if let syntax.
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
