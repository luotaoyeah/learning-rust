/*
  Patterns and Matching
      Pattern Syntax
          Destructuring to Break Apart Values
              Destructuring Structs
*/

pub fn fn_18_03_02_01() {
    struct Point {
        x: i32,
        y: i32,
    }

    println!("-------------------------------------------------- 01");
    {
        /*
          除了可以使用 pattern 对 tuple 进行 destructure 之外，
          还可以对 struct 进行 destructure；
        */

        let p = Point { x: 9, y: 9 };
        let Point { x: a, y: b } = p;
        println!("{} {}", a, b); // 9 9
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           当 pattern 中的变量名跟 struct 中的 field 名一样时，
           可以省略变量名；
        */

        let p = Point { x: 9, y: 9 };
        let Point { x, y } = p;
        println!("{} {}", x, y); // 9 9
    }

    println!("-------------------------------------------------- 03");
    {
        /*
          pattern 中可以指定某些 field 的值来进行匹配，
          而对其他的 field 进行 destructure；
        */

        let p = Point { x: 0, y: 9 };

        match p {
            Point { x, y: 0 } => println!("x: {}", x),
            Point { x: 0, y } => println!("y: {}", y), // y: 9
            Point { x, y } => println!("{} {}", x, y),
        }
    }
}
