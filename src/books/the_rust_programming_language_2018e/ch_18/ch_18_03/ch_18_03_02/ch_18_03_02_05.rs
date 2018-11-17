/*
  Patterns and Matching
      Pattern Syntax
          Destructuring to Break Apart Values
              Destructuring Structs and Tuples
*/

pub fn fn_18_03_02_05() {
    println!("-------------------------------------------------- 01");
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let ((a, b), Point { x, y }) = ((2, 8), Point { x: 3, y: 9 });
        println!("{} {} {} {}", a, b, x, y); // 2 8 3 9
    }
}
