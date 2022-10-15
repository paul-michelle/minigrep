use std::{env, error::Error, fs};

const QUERY_STRING_INDEX: usize = 1;
const TARGET_FILE_PATH_INDEX: usize = 2;
const MIN_ARGS_EXPECTED_COUNT: usize = 3;
const IGNORE_CASE_ENVVAR_NAME: &str = "IGNORE_CASE";

pub struct Config {
    query_string: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < MIN_ARGS_EXPECTED_COUNT {
            return Err("At least query string and target file path expected, \
            e.g.: ./minigrep Savannah poem.txt");
        }

        let query_string = args[QUERY_STRING_INDEX].clone();
        let file_path = args[TARGET_FILE_PATH_INDEX].clone();
        let ignore_case = match env::var(IGNORE_CASE_ENVVAR_NAME) {
            Ok(val) => val.eq_ignore_ascii_case("true") || val.eq("1"),
            Err(_) => false,
        };

        Ok(Config {
            query_string,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.query_string.is_empty() {
        return Ok(());
    }

    let file_contents = fs::read_to_string(&config.file_path)?;

    for line in search(&config.query_string, &file_contents, config.ignore_case) {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut matches: Vec<&str> = Vec::new();

    if !ignore_case {
        for line in contents.lines() {
            if line.contains(query) {
                matches.push(line)
            }
        }
        return matches;
    }

    let q = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&q) {
            matches.push(line)
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query_string = "duct";
        let file_contents =
            "\nConduct\nRust:\nsafe, fast, corrupted duck-typing,\nnatural, intuitive, productive.\nDuct case sensitive";
        assert_eq!(
            vec!["Conduct", "natural, intuitive, productive."],
            search(query_string, file_contents, false)
        )
    }

    #[test]
    fn test_search_case_insensitive() {
        let query_string = "DuCT";
        let file_contents =
            "\nConduct\nRust:\nsafe, fast, corrupted duck-typing,\nnatural, intuitive, productive.";
        assert_eq!(
            vec!["Conduct", "natural, intuitive, productive."],
            search(query_string, file_contents, true)
        )
    }
}
