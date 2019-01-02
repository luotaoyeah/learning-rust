/*
  Functional Language Features: Iterators and Closures
      Closures: Anonymous Functions that Can Capture Their Environment
          Capturing the Environment with Closures
*/

pub fn fn_13_01_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          closure 比 function 多的一个功能：捕获变量；
        */

        let x = 9;

        /*
          在 closure 的 body 中使用并捕获了当前 scope 中的变量 x；
        */
        let equal_to_x = |a| a == x;
        let y = 9;
        println!("{}", equal_to_x(y)); // true
    }

    println!("-------------------------------------------------- 02");
    {
        let x = 9;
        fn equal_to_x(a: u32) -> bool {
            a == x // [E0434]: can't capture dynamic environment in a fn item
        }
        let y = 9;
        println!("{}", equal_to_x(y));
    }
}
