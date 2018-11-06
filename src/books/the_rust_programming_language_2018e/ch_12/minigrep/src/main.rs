use std::env;
use std::fs;

fn main() {
    // 获取命令行参数列表
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args);

    // 读取文件内容
    let text = fs::read_to_string(config.filename).expect("文件读取失败");
    println!("{}", text);
}

/// 配置
struct Config {
    query: String,
    filename: String,
}

/// 解析参数
/// * `args` 参数列表
fn parse_config(args: &[String]) -> Config {
    let query: String = args[1].clone();
    let filename: String = args[2].clone();

    Config { query, filename }
}
