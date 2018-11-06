use std::env;
use std::error::Error;
use std::fs;

/// 运行
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let text = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &text)
    } else {
        search_case_insensitive(&config.query, &text)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

/// 配置
pub struct Config {
    /// 要查找的字符串
    query: String,
    /// 文件名
    filename: String,
    /// 是否区分大小写
    case_sensitive: bool,
}

impl Config {
    /// 解析参数
    /// * `args` 参数列表
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        let case_sensitive = env::var("MINIGREP_CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

/// 查找：区分大小写
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

/// 查找：不区分大小写
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();

    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
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
Pick three.\
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.\
";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
