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

    let query_string = &args[1];
    let file_path    = &args[2];

    println!("Searching for {} in file {}\n", &query_string, &file_path);

    let file_contents = match fs::read_to_string(&file_path) {
        Ok(contents) => contents,
        Err(_) => String::from("File not found.")
    };

    println!("{}", file_contents)
}
