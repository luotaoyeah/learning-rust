/*
   Writing Automated Tests
       Test Organization
           Unit Tests
               The Tests Module and #[cfg(test)]
 */

pub fn fn_11_03_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 将测试分为两类：单元测试（unit test），集成测试（integrated test）；
         */
    }
}

/*
    unit test 的代码通常跟实际的代码放在同一个文件，
    通常放在 tests 模块下面，通常使用 #[cfg(tests)] 进行标注；
    #[cfg(tests)] 表示该模块中的代码只有在执行 cargo test 命令时才会编译和执行，
    在执行 cargo build 命令时不会编译和执行，不会打包到最终的构建文件中；
 */
#[cfg(test)]
mod tests {
    #[test]
    fn test_01() {}

    #[test]
    fn test_02() {}
}
