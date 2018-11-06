use std::env;
use std::fs;
use std::process;

fn main() {
    // 获取命令行参数列表
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|e| {
        println!("参数解析失败：{}", e);
        process::exit(1);
    });

    // 读取文件内容
    let text = fs::read_to_string(config.filename).expect("文件读取失败");
    println!("{}", text);
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
            return Err("NOT ENOUGH ARGUMENTS");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
