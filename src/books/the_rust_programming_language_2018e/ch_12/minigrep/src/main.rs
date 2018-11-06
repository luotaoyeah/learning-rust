use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    // 获取命令行参数列表
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|e| {
        println!("[ERROR] 参数解析失败：{}", e);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("[ERROR] 程序运行失败：{}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 读取文件内容
    let text = fs::read_to_string(config.filename)?;
    println!("{}", text);

    Ok(())
}

/// 配置
struct Config {
    query: String,
    filename: String,
}

impl Config {
    /// 解析参数
    /// * `args` 参数列表
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
