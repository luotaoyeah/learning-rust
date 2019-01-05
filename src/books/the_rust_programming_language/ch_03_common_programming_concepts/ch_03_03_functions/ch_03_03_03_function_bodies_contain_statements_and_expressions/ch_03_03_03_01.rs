/*
  Functions
      Function Bodies Contain Statements and Expressions
*/

pub fn fn_03_03_03_01() {
    println!("-------------------------------------------------- 01");
    // function body 中可以包含 statement 和 expression
    // statement 就是一系列的操作，它不会有返回值
    // expression 会有返回值，可以赋值给其他变量

    // 函数定义本身（function definition）就是一个 statement
    fn fn_01() {
        // 变量声明以及赋值也是一个 statement
        let x: i32 = 9;
    }
}
