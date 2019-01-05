/*
  Functions
      Function Bodies Contain Statements and Expressions
          expression has a return value
*/

pub fn fn_03_03_03_03() {
    println!("-------------------------------------------------- 01");
    // expression 有返回值，它可以赋值给其他变量

    // 算术运算是一个 expression
    let sum: i32 = 9 + 9;

    println!("-------------------------------------------------- 02");
    // macro 的调用，也是一个 expression
    let r = println!("{}", sum); // 18
    println!("{:?}", r); // ()

    println!("-------------------------------------------------- 03");
    // function 的调用，也是一个 expression
    fn fn_01() {}
    fn fn_02() -> i32 {
        9
    }

    // 对于没有定义返回值的函数，其返回值为 ()
    println!("{:?}", fn_01()); // ()
    println!("{:?}", fn_02()); // 9
}
