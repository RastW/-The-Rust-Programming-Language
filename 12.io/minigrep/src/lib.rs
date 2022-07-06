use std::error::Error;
use std::{fs, vec};

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(list: &[String]) -> Result<Config, &'static str> {
        if list.len() < 3 {
            return Err("not enough arguments");
        }
        let query = list[1].clone();
        let filename = list[2].clone();
        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line);
        }
    }
    // search(&config.query, &contents);
    // println!("{}", contents);
    Ok(())
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe, fast, productive.";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
