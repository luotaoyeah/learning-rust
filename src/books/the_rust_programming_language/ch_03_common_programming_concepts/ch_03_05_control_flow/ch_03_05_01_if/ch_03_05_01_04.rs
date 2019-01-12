/*
  Control Flow
      if Expressions
         Using if in a let Statement
*/

pub fn fn_03_05_01_04() {
    println!("-------------------------------------------------- 01");

    // 因为 if expression 是一个表达式
    // 因此它可以赋值给某个变量
    // 这种用法类似于其它语言中的三元表达式（c?a:b）

    let b: bool = false;
    let i: i32 = if b { 9 } else { 99 };
    println!("{}", i); // 9
}
