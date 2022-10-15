use std::env;
use std::fs;

const ARGS_EXPECTED_COUNT: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\nThe {} has been executed...\n" , &args[0]);

    if args.len() < ARGS_EXPECTED_COUNT {
        println!("Query strings and target file path expected");
        return;
    }

    let config = Config::new(&args);
   
    println!("Searching for {} in file {}\n", config.query_string, config.file_path);

    let file_contents = match fs::read_to_string(&config.file_path) {
        Ok(contents) => contents,
        Err(_) => String::from("File not found.")
    };

    println!("{}", file_contents)
}


struct Config{
    query_string: String,
    file_path: String
}

impl Config {
    fn new(args: &Vec<String>) -> Self {
        Config {
            query_string: args[1].clone(),
            file_path: args[2].clone()
        }
    }
}
