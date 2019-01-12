/*
  Functions
      Functions with Return Values
*/

pub fn fn_03_03_04_01() {
    println!("-------------------------------------------------- 01");

    println!("{}", five()); // 5
}

// function 的返回类型使用 -> T 的形式来声明
fn five() -> i32 {
    // 如果函数体中最后是一个 expression，
    // 那么该 expression 就会隐式地成为该函数的返回值
    5
}
