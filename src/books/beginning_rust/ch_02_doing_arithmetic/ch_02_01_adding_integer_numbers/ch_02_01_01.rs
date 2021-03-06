/*
  Doing Arithmetic
      Adding Integer Numbers
*/

pub fn fn_02_01_01() {
    println!("-------------------------------------------------- 01");
    // 如果开启了编译优化，则结果 114 会以二进制的形式，存储在编译后的结果中
    // 如果没有开启编译优化，则 80 和 34 分别会以二进制的形式，存储在编译后的结果中，
    // 同时，+ 操作符也会被存储在结果中
    println!("{}", 80 + 34); // 114
    println!("{} + {} = {}", 80, 34, 80 + 34); // 80 + 34 = 114
}
