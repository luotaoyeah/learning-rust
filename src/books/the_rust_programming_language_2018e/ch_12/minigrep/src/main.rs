extern crate minigrep;

use minigrep::Config;
use std::env;
use std::process;

fn main() {

    let config: Config = Config::new(env::args()).unwrap_or_else(|e| {
        eprintln!("[ERROR] 参数解析失败：{}", e);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("[ERROR] 程序运行失败：{}", e);
        process::exit(1);
    }
}
