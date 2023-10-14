use std::error::Error;
use std::{fs,env};

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();

        let query =  match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive_str = match args.next() {
            Some(args) => args,
            None => "true".to_string(),
        };

        let case_sensitive = match case_sensitive_str.parse::<bool>(){
            Ok(value) => value,
            Err(_) => return Err("Invalid value for case_sensitive, should be 'true' or 'false'"),
        };

        if let Some(_) = args.next(){
            return Err("Too much arguments")
        };

        Ok(Config{query, filename, case_sensitive})
    }
}


pub fn run(config: Config) ->Result<(), Box<dyn Error>>{
    let contents= fs::read_to_string(config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result{
        println!("{}",line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();  // 一个string
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
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
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
            "测试失败"
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}