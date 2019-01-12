/*
  Control Flow
      if Expressions
         condition must be a bool
*/

pub fn fn_03_05_01_02() {
    println!("-------------------------------------------------- 01");

    let n: i32 = 6;

    // if 后面的条件必须是一个 bool 类型的表达式，
    // 否则编译报错

    /*
        if n { // expected bool, found i32
            println!("{}", "ERR");
        }
    */
}
