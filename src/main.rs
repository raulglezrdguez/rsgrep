use std::env;
use std::process;

use rsgrep::Config;
use rsgrep::run;

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


