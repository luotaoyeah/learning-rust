/*
  Doing Arithmetic
      Breaking Literal Strings
*/

pub fn fn_02_05_02() {
    println!("-------------------------------------------------- 01");
    {
        // 当一个 string literal 跨越多行时，如果不想保留中间的空白符和换行符，
        // 可以在行尾使用一个 \

        // THERE IS ONLY ONE LINE
        println!(
            "{}",
            "THERE \
             IS \
             ONLY \
             ONE \
             LINE"
        );
    }
}
