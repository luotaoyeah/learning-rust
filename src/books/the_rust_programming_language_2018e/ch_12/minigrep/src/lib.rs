use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// 运行
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let text = fs::read_to_string(config.filename)?;
    println!("{}", text);

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
