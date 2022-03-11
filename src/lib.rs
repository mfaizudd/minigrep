use std::{env, error::Error, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duck";
        let contents = "\
From troubles of the world
I turn to ducks,
Beautiful comical things
Duck supremacy";

        assert_eq!(vec!["I turn to ducks,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCk";
        let contents = "\
From troubles of the world
I turn to ducks,
Beautiful comical things
Duck supremacy";

        assert_eq!(
            vec!["I turn to ducks,", "Duck supremacy"],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = if !env::var("CASE_INSENSITIVE").is_err() {
            env::var("CASE_INSENSITIVE").is_err()
        } else {
            args.get(3).is_none()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
