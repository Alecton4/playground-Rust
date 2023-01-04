use std::env; // To take environment variables
use std::error::Error;
use std::fs; // To read files.

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // NOTE: expect() will panic.
    // let contents = fs::read_to_string(config.filename).expect("Cannot read file into string!");

    // NOTE: "?" here mean if read_to_string() returns an Error type,
    // then this Error type is automatically be returned from this run() function.
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(config.query.as_str(), contents.as_str())
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// TIP: In powershell, run the following command:
// $env:CASE_INSENSITIVE='true'; cargo run the poem.txt; Remove-Item Env:\CASE_INSENSITIVE
// TODO: Implement this function using search().
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // NOTE: Panic is more appropriate for programming errors rather than usage errors
        // if args.len() < 3 {
        //     panic!("Not enough arguments!");
        // }

        // if args.len() < 3 {
        //     Err("Not enough arguments!") // NOTE: This str literal have static lifetime.
        // } else {
        //     Ok(Config {
        //         query: args[1].clone(),
        //         filename: args[2].clone(),
        //         case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        //     })
        // }

        let mut args = args.skip(1);
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query provided!"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename provided!"),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
