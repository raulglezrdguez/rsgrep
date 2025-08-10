use std::env;
use std::process;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    // if args.len() != 3 {
    //     eprintln!("Usage: {} <query> <file_path>", args[0]);
    //     process::exit(1);
    // }

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        eprintln!("Usage: {} <query> <file_path>", args[0]);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    let content = fs::read_to_string(config.file_path).expect("read file content");
    println!("With text:\n{content}");
}

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
