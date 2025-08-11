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
    let found = search(config.query, &content);
    if found.len() > 0 {
        println!("Found:");
        for line in found {
            println!("{line}")
        }
    } else {
        println!("'{}' not found in '{}'", config.query, config.file_path);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_build() {
        let args = vec!["rsgrep".to_string(), "test".to_string(), "test.txt".to_string()];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "test");
        assert_eq!(config.file_path, "test.txt");
    }

    #[test]
    fn test_config_build_invalid_args() {
        let args = vec!["rsgrep".to_string()];
        assert!(Config::build(&args).is_err());
    }

    #[test]
    fn test_search() {
        let query = "another";
        let contents = "\
        This is a test string.\nThis is a another string.\nAnd another one.
        ";
        let results = search(query, contents);
        let result_ok: Vec<& str> = vec![];
        assert_eq!(results, result_ok); 
    }

}