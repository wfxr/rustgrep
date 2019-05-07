/*******************************************************************************
*    Author: Wenxuan                                                           *
*     Email: wenxuangm@gmail.com                                               *
*   Created: 2018-05-01 22:31                                                  *
*******************************************************************************/

extern crate core;

use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search_with_query(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: args.next().ok_or("query not set")?,
            filename: args.next().ok_or("filename not set")?,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn search_with_filter<P>(contents: &str, predicate: P) -> Vec<&str> where P: Fn(&&str) -> bool
{
    contents.lines().filter(predicate).collect()
}

pub fn search_with_query<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    match case_sensitive {
        true => search_with_filter(contents, |l| l.contains(query)),
        false => {
            let query = &query.to_lowercase();
            search_with_filter(contents, |l| l.to_lowercase().contains(query))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_with_query(query, contents, true));
    }

    #[test]
    fn no_result() {
        let query = "abcdefg";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let empty: Vec<&str> = Vec::new();
        assert_eq!(empty, search_with_query(query, contents, true));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search_with_query(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
Duct tape.";

        assert_eq!(vec!["Rust:", "Trust me."], search_with_query(query, contents, false));
    }
}
