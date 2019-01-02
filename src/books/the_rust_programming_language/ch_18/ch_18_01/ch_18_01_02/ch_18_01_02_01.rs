/*
  Patterns and Matching
      All the Places Patterns Can Be Used
          Function Parameters
*/

pub fn fn_18_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          function parameters 也是 pattern；
        */

        fn fn_01(x: i32) {
            println!("{}", x);
        }

        fn_01(9); // 9

        /*
          function parameter 中可以使用 tuple destructure；
        */
        fn fn_02(&(x, y): &(i32, i32)) {
            println!("{} {}", x, y);
        }

        fn_02(&(3, 9)); // 3 9
    }
}
