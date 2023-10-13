use std::error::Error;
use std::fs;

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        } else if args.len() == 3{
            let query = args[1].clone();
            let filename = args[2].clone();
            let case_sensitive = true;
            Ok(Config{query, filename, case_sensitive})
        } else if args.len() == 4{
            let query = args[1].clone();
            let filename = args[2].clone();
            // parse::<bool>() 方法对于 "false" 和 "true" 字符串以及其他有效的布尔表示形式（例如 "0" 和 "1"）都有效。
            let case_sensitive = match args[3].clone().parse::<bool>(){
                Ok(parsed)=>parsed,
                Err(_)=> return Err("invalid value for case_sensitive"),
            };
            Ok(Config{query, filename, case_sensitive})
        } else {
            return Err("extra arguments")
        }

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