/*
  Control Flow
      if Expressions
         Handling Multiple Conditions with else if
*/

pub fn fn_03_05_01_03() {
    println!("-------------------------------------------------- 01");

    // if...else if...else if...else 可以连续使用
    let n: i32 = 90;

    if n < 0 {
        println!("{}", "0)");
    } else if n < 10 {
        println!("{}", "[0, 10)");
    } else if n < 100 {
        println!("{}", "[10, 100)");
    } else {
        println!("{}", "[100");
    }
}
