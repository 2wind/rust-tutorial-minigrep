use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub inverse: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let inverse = args.get(3);
        let inverse = match inverse {
            Some(i) => match i.as_str() {
                "inverse" => true,
                "v" => true,
                _ => false,
            },
            None => false,
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
            inverse,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents, config.inverse)
    } else {
        search(&config.query, &contents, config.inverse)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str, inverse: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        let mut matching = line.contains(query);
        if inverse {
            matching = !matching;
        }

        if matching {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str, inverse: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        let mut matching = line.to_lowercase().contains(&query);
        if inverse {
            matching = !matching;
        }

        if matching {
            results.push(line);
        }
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
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents, false)
        );
    }

    #[test]
    fn case_sensitive_inverse() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:", "Pick three."], search(query, contents, true));
    }

    #[test]
    fn case_insensitive_inverse() {
        let query = "RusT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search_case_insensitive(query, contents, true)
        );
    }
}
