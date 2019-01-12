/*
  Doing Arithmetic
      Other Operations between Integer Numbers
*/

pub fn fn_02_02_01() {
    println!("-------------------------------------------------- 01");
    // integer number 之间支持四则运算和余运算
    // 因为下面的整个表达式在编译时期就可以确定结果，
    // 因此 rust 在编译的时候，就会将计算结果直接存储到编译结果中，
    // 提高运行时期的性能
    println!("{}", (23 - 6) % 5 + 20 * 30 / 7); // 87
}
