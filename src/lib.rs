use std::fs;            // for the file system
use std::error::Error;  // for the error handling
use std::env;           // for the env variables

pub struct Config
{
    pub m_query: String,
    pub m_filename: String,
    pub m_case_sensitive: bool,
}

impl Config
{
    pub fn new(args: &[String]) -> Result<Config,&str>
    {
        if args.len() < 3
        {
            return Err("not enough arguments");
        }

        // Creates copies of the args
        // better to uses life times
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { m_query: query, m_filename: filename, m_case_sensitive: case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.m_filename)?;

    let results = if config.m_case_sensitive
    {
        search(&config.m_query, &contents)
    }
    else
    {
        search_case_insensitive(&config.m_query, &contents)
    };

    for line in results
    {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three
Duct tape";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }

    #[test]
    fn test_insesitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(query, contents))
    }

}