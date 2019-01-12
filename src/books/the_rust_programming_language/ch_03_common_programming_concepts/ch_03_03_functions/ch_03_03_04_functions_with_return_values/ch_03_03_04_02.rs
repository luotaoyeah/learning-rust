/*
  Functions
      Functions with Return Values
          return
*/

pub fn fn_03_03_04_02() {
    println!("-------------------------------------------------- 01");
    println!("{}", five()); // 9
}

// 可以在函数体中使用 return 提前返回
fn five() -> i32 {
    return 9;
    5
}
