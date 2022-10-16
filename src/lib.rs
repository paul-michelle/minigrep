use std::{env, error::Error, fs};

const IGNORE_CASE_ENVVAR_NAME: &str = "IGNORE_CASE";

pub struct Config {
    query_string: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query_string = match args.next() {
            Some(val) => val,
            None => return Err("Query string not provided."),
        };

        let file_path = match args.next() {
            Some(val) => val,
            None => return Err("Target file path not provided."),
        };

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
    if !ignore_case {
        return contents
            .lines()
            .filter(|line| line.contains(query))
            .collect();
    }

    let q = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&q))
        .collect()
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
