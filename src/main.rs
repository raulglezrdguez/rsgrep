use std::env;
use std::process;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage: {} <query> <file_path>", args[0]);
        process::exit(1);
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{}'", query);
    println!("In file '{}'", file_path);

}
