/*
   An I/O Project: Building a Command Line Program
       Accepting Command Line Arguments
           Reading the Argument Values
 */

use std::env;

pub fn fn_12_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 std::env::args() 方法读取命令行参数；
         */
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
    }
}
