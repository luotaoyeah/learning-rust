/*
  Doing Arithmetic
      Breaking Literal Strings
*/

pub fn fn_02_05_01() {
    println!("-------------------------------------------------- 01");
    {
        // 一个 string literal 可以跨越多行
        // 此时，中间所有的字符都会被原样保留，包括换行符和空白符

        // there
        //            are
        //            three lines
        println!(
            "{}",
            "there
            are
            three lines"
        );
    }
}
