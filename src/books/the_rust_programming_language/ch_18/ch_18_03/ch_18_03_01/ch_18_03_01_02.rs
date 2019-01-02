/*
  Patterns and Matching
      Pattern Syntax
          Matching Named Variables
*/

pub fn fn_18_03_01_02() {
    println!("-------------------------------------------------- 01");
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(10) => println!("10"),
            /*
              这儿的 y 是当前 scope 中的 y，而不是外面的 y；
            */
            Some(y) => println!("{:?}", y), // 5
            /*
              这儿的 x 是外面的 x；
            */
            _ => println!("{:?}", x),
        }

        println!("{:?} {:?}", x, y); // Some(5) 10
    }
}
