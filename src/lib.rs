use std::fs;
use std::error::Error;

// Config struct to hold the query and file path
pub struct Config <'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl <'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() != 3 {
            return Err("Invalid argument count");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    if content.contains(config.query) {
        println!("Found '{}' in '{}'", config.query, config.file_path);
    } else {
        println!("'{}' not found in '{}'", config.query, config.file_path);
    }
    Ok(())
}

