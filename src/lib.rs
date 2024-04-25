use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /* let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file"); */
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // println!("With text:/n{contents}");
    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        /* if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone(); */
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file_path string"),
        };

        let ignore_case = match env::var("IGNORE_CASE").ok() {  
            // 环境变量优先级最高
            // 如果环境变量存在，并且值为 "1"  
            Some(ref s) if s == "1" => true,  
            // 如果环境变量存在，并且值为 "0"  
            Some(ref s) if s == "0" => false,
            // 如果环境变量不存在或为其他值，则检查命令行参数  
            _ => match args.next() {  
                // 如果命令行参数存在，并且不是 "0"  
                Some(flag) => flag != "0",  
                // 如果没有更多的命令行参数，则默认为 false  
                None => false,  
            },  
        };
/*         let ignore_case = match env::var("IGNORE_CASE").ok() {
            Some(0) | None => match args.next() {
                Some(flag) => flag != "0",
                None => false,
            },
            Some(1) => true,
        }; */

        /* let ignore_case_flag_arg = match args.next() {
            Some(arg) => Some(arg),
            None => return Err("Didn't get a ignore_case_flag_arg string"),
        };
  
        // 获取 IGNORE_CASE 环境变量  
        let ignore_case_env = env::var("IGNORE_CASE").ok();  
      
        let ignore_case = match (ignore_case_env, ignore_case_flag_arg) {  
            (Some("0"), _) | (_, None) => false, // 环境变量为 "0" 或没有第四个参数时，忽略大小写为 false  
            (Some(_), _) | (_, Some("1")) => true, // 环境变量为其他值或第四个参数为 "1" 时，忽略大小写为 true  
            _ => return Err("Invalid ignore_case flag or environment variable"), // 其他情况返回错误  
        }; */
        // let ignore_case = env::var("IGNORE_CASE").is_ok();
/*         let ignore_case_flag = env::var("IGNORE_CASE").ok();
        let ignore_case = match ignore_case_flag.as_ref().map(String::as_ref) {
            None => {
                if args.count() == 4 {
                    let flag_ignore_case: String = match args.next().clone() {
                        Some(arg) => arg,
                        None => return Err("Didn't get a file_path string"),
                    };
                    match flag_ignore_case == "1" {  
                        true => true,  
                        _ => false,  
                    }
                } else {
                    false
                }
            },
            Some("0") => false,
            Some(_) => true,
        }; */

        // 获取 IGNORE_CASE 环境变量  
        // let ignore_case = env::var("IGNORE_CASE").is_ok(); 

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents.lines().filter(|line| line.contains(query)).collect()
    /* let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results */
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
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
