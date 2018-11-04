/*
   Writing Automated Tests
       Controlling How Tests Are Run
           Running Tests in Parallel or Consecutively
 */

pub fn fn_11_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           cargo test 命令会执行两个动作：
               在 test 模式下编译代码，
               执行编译好的二进制文件；
           两个动作的命令行参数使用 -- 分隔；
         */

        /*
           默认情况，多个测试会多线程并行执行，
           可以通过 -- --test-threads=n 指定线程数量；
         */
    }
}
