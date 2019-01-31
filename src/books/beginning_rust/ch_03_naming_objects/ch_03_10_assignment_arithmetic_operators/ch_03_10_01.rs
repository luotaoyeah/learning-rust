/*
  Naming Objects
      3.10. Assignment Arithmetic Operators
*/

pub fn fn_03_10_01() {
    println!("-------------------------------------------------- 01");
    // 几个算术运算符号的简写形式

    let mut n01: i32 = 9;
    n01 = n01 + 9;
    n01 = n01 - 9;
    n01 = n01 * 9;
    n01 = n01 / 9;
    println!("{}", n01); // 9

    n01 += 9;
    n01 -= 9;
    n01 *= 9;
    n01 /= 9;
    println!("{}", n01); // 9
}
