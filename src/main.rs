use std::env;
use std::process;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage: {} <query> <file_path>", args[0]);
        process::exit(1);
    }

    let config = parse_config(&args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let content = fs::read_to_string(config.file_path).expect("read file content");
    println!("With text:\n{content}");
}

struct Config <'a> {
    query: &'a str,
    file_path: &'a str,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

    Config {query, file_path}
}