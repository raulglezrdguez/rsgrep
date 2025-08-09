use std::env;
use std::process;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage: {} <query> <file_path>", args[0]);
        process::exit(1);
    }

    let (query, file_path) = parse_config(&args);

    println!("Searching for '{}'", query);
    println!("In file '{}'", file_path);

    let content = fs::read_to_string(file_path).expect("read file content");
    println!("With text:\n{content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}