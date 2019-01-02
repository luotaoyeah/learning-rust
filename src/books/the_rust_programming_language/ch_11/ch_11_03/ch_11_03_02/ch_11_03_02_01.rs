/*
   Writing Automated Tests
       Test Organization
           Integration Tests
 */

pub fn fn_11_03_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           integrated test 通常放在单独的文件夹中，通常为根目录下的 tests 目录；
           integrated test 不需要使用 #[cfg(test)] 声明特殊的模块，
               只需要使用 #[test] 标注测试方法；

           依然可以通过指定测试方法名称进行过滤，
           或者通过 cargo test --test inte_test_01 参数指定运行某个测试文件；
         */
    }
}
