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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't gate a file name"),
        };
        let case_sensitive = if !env::var("CASE_INSENSITIVE").is_err() {
            env::var("CASE_INSENSITIVE").is_err()
        } else {
            args.next().is_none()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
