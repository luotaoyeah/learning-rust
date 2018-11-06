use std::env;
use std::fs;

fn main() {
    // 获取命令行参数列表
    let args: Vec<String> = env::args().collect();

    // 将需要的参数保存到变量
    let query: &String = &args[1];
    let filename: &String = &args[2];

    // 读取文件内容
    let text = fs::read_to_string(filename).expect("文件读取失败");
    println!("{}", text);
}
