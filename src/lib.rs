use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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
        println!("{}: {}", line.number, line.text);
    }

    Ok(())
}

pub struct MatchingLine<'a> {
    number: i32,
    text: &'a str,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<MatchingLine<'a>> {
    let mut results: Vec<MatchingLine> = Vec::new();
    let mut line_number = 1;

    for line in contents.lines() {
        if line.contains(query) {
            results.push(MatchingLine {
                number: line_number,
                text: line,
            });
        }
        line_number += 1;
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<MatchingLine<'a>> {
    let mut results: Vec<MatchingLine> = Vec::new();
    let mut line_number = 1;

    for line in contents.lines() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) {
            results.push(MatchingLine {
                number: line_number,
                text: line,
            });
        }
        line_number += 1;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        let matching_line = MatchingLine {
            number: 2,
            text: "safe, fast, productive.",
        };
        let result = search(query, contents);

        assert_eq!(matching_line.number, result[0].number);
        assert_eq!(matching_line.text, result[0].text);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let matching_line = MatchingLine {
            number: 1,
            text: "Rust:",
        };
        let result = search_case_insensitive(query, contents);

        assert_eq!(matching_line.number, result[0].number);
        assert_eq!(matching_line.text, result[0].text);
    }
}
