/*
  Naming Objects
      3.7. Boolean Expression
*/

pub fn fn_03_07_01() {
    println!("-------------------------------------------------- 01");
    // 布尔值之间可以进行逻辑运算（与或非）

    let b01: bool = true;
    let b02: bool = false;

    println!("{} {} {}", b01 && b02, b01 || b02, !b01); // false true false
}
