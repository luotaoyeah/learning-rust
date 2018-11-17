/*
  Patterns and Matching
      Pattern Syntax
          Ignoring Values in a Pattern
              Ignoring an Entire Value with _
*/

pub fn fn_18_03_03_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          _ 作为 pattern 可以匹配任意值，除了可以作为 match 的最后一个 arm 之外，
          还可以作为函数的参数，表示这个参数不会被使用；
        */

        fn fn_01(_: i32, y: i32) {
            println!("{}", y);
        }

        fn_01(3, 9); // 9
    }
}
