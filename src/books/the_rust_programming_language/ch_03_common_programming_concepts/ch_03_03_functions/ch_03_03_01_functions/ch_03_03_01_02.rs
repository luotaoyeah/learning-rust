/*
  Functions
      Functions
        declaration position
*/

fn fn_01() {
    println!("{}", "FN01");
}

pub fn fn_03_03_01_02() {
    println!("-------------------------------------------------- 01");

    // 函数可以定义在调用者的前面，
    // 也可以定义在调用者的后面
    fn_01();
    fn_02();
}

fn fn_02() {
    println!("{}", "FN02");
}
