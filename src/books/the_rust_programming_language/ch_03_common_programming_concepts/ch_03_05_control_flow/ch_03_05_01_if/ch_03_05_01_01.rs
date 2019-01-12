/*
  Control Flow
      if Expressions
*/

pub fn fn_03_05_01_01() {
    println!("-------------------------------------------------- 01");

    let n: i32 = 6;

    // 条件满足时，执行 if 语句块中的代码，
    // 条件不满足时，执行 else 语句块中的代码
    if n < 9 {
        println!("{}", "TRUE");
    } else {
        println!("{}", "FALSE");
    }
}
