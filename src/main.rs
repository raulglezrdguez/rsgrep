use std::env;
use std::process;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        eprintln!("Usage: {} <query> <file_path>", args[0]);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    println!("Search completed successfully.");
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config.file_path)?;
    if content.contains(config.query) {
        println!("Found '{}' in '{}'", config.query, config.file_path);
    } else {
        println!("'{}' not found in '{}'", config.query, config.file_path);
    }
    Ok(())
}

// Config struct to hold the query and file path
struct Config <'a> {
    query: &'a str,
    file_path: &'a str,
}

impl <'a> Config<'a> {
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() != 3 {
            return Err("Invalid argument count");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}
