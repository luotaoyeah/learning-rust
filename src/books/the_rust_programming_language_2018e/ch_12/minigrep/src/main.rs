extern crate minigrep;

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // 获取命令行参数列表
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|e| {
        println!("[ERROR] 参数解析失败：{}", e);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("[ERROR] 程序运行失败：{}", e);
        process::exit(1);
    }
}
