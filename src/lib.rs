use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enoguh arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().into_iter().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().into_iter().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "tap";
        let contents = "lolz \n
        trolllz \ntap colts";

        assert_eq!(vec!["tap colts"], search(query, contents))
    }

    #[test]
    fn no_result() {
        let query = "b";
        let contents = "c";

        let empty_result: Vec<String> = vec![];

        assert_eq!(empty_result, search(query, contents))
    }

    #[test]
    fn case_sensitive_search_works() {
        let query = "Abc";
        let contents = "abc";

        let empty_result: Vec<String> = vec![];

        assert_eq!(empty_result, search(query, contents))
    }

    #[test]
    fn case_insensitive_search_works() {
        let query = "Abc";
        let contents = "abc";

        assert_eq!(vec!["abc"], search_case_insensitive(query, contents))
    }
}