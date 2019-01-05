/*
  Functions
      Functions Parameters
*/

pub fn fn_03_03_02_01() {
    println!("-------------------------------------------------- 01");
    fn_01(9, "99"); // X: 9, Y: "99"
}

// 函数可以定义参数
fn fn_01(x: i32, y: &str) {
    println!("X: {}, Y: {:?}", x, y);
}
