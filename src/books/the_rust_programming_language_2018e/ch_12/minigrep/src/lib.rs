use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// 运行
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let text = fs::read_to_string(config.filename)?;
    Ok(())
}

/// 配置
pub struct Config {
    query: String,
    filename: String,
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

        Ok(Config { query, filename })
    }
}

/// 查找
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
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
Pick three.\
";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
