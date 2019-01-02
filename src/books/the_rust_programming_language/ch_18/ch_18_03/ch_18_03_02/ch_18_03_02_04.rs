/*
  Patterns and Matching
      Pattern Syntax
          Destructuring to Break Apart Values
              Destructuring References
*/

pub fn fn_18_03_02_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          当要 destructure 的值是一个 reference 时，
          pattern 需要使用 & 开头，将 reference 指向的值解构到变量中；
        */

        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];

        /*
          这儿发生了两次解构，
          第一次是 reference destructure，
          第二次是 struct destructure；
        */
        let sum: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
        println!("{}", sum);
    }
}
