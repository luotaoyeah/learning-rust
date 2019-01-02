/*
  Patterns and Matching
      Pattern Syntax
          Ignoring Values in a Pattern
              Ignoring Remaining Parts of a Value with ..
*/

pub fn fn_18_03_03_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          .. 用来匹配剩下的值；
        */

        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let point = Point { x: 1, y: 2, z: 3 };
        let Point { x, .. } = point;
        println!("{}", x); // 1

        /*
          如果不使用 .. 而是使用 _ 的话，则会非常繁琐；
        */
        let Point { x, y: _, z: _ } = point;
        println!("{}", x); // 1
    }

    println!("-------------------------------------------------- 02");
    {
        let t = (1, 2, 3, 4, 5);
        /*
          .. 用在 tuple 的中间
        */
        let (a, .., e) = t;
        println!("{} {}", a, e); // 1 5
    }
}
