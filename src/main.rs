use std::{env, fs, process, error::Error};


const QUERY_STRING_INDEX: usize      = 1;
const TARGET_FILE_PATH_INDEX: usize  = 2;
const MIN_ARGS_EXPECTED_COUNT: usize = 3;

struct Config{
    query_string: String,
    file_path: String
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < MIN_ARGS_EXPECTED_COUNT { 
            return Err("At least query string and target file path expected, \
            e.g.: ./minigrep Savannah poem.txt")
        }
        Ok(Config {
            query_string: args[QUERY_STRING_INDEX].clone(),
            file_path: args[TARGET_FILE_PATH_INDEX].clone()
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&config.file_path)?;
    println!("{}", file_contents);
    Ok(())
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = Config::build(&argv).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Failed to read file contents: {}", e);
        process::exit(1)
    }
}
