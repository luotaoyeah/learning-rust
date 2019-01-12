/*
  Functions
      Functions with Return Values
*/

pub fn fn_03_03_04_03() {
    println!("-------------------------------------------------- 01");
    println!("{}", plus_one(5)); // 6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
