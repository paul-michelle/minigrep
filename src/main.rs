use std::env;

const ARGS_EXPECTED_COUNT: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("The {} has been executed..." , &args[0]);

    if args.len() < ARGS_EXPECTED_COUNT {
        println!("Query strings and target file path expected");
        return;
    }

    let query_string = &args[1];
    let file_path    = &args[2];

    println!("Searching for {} in file {}", &query_string, &file_path);
}
