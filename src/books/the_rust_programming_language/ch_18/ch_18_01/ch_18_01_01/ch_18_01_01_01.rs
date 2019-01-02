/*
  Patterns and Matching
      All the Places Patterns Can Be Used
          match Arms
*/

pub fn fn_18_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          pattern 可以用在 match 表达式的 arm 中；
        */

        let x: i32 = 9;

        /*
          _ 是一种特殊的 pattern，它可以匹配任意值，
          且它不会绑定数据，通常用在 match 表达式的最后一个 arm 中；
        */
        match x {
            9 => {
                println!("9");
            }
            _ => (),
        }
    }
}
