/*
  Doing Arithmetic
      Breaking Literal Strings
*/

pub fn fn_02_05_03() {
    println!("-------------------------------------------------- 01");
    {
        // 当一个 string literal 跨越多行时，如果想保留换行符，但是不保留空白符，
        // 可以使用 \n

        // there
        // are
        // there lines
        println!(
            "{}",
            "there\n\
             are\n\
             there lines"
        );
    }
}
