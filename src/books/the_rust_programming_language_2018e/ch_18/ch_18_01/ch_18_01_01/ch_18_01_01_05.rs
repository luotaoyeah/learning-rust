/*
  Patterns and Matching
      All the Places Patterns Can Be Used
          let Statements
*/

pub fn fn_18_01_01_05() {
    println!("-------------------------------------------------- 01");
    {
        /*
          let 赋值语句中的 x 也是一个 pattern；
        */

        let x = 5;
        println!("{}", x); // 5

        /*
          let 赋值语句中使用 tuple destructure，
          其中 (x, y, z) 也是一个 pattern；
        */

        let (x, y, z) = (1, 2, 3);
        println!("{} {} {}", x, y, z); // 1 2 3
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          当 tuple destructure 中的 pattern 不匹配 tuple 中元素的个数时，编译失败；
        */

        /*
                let (x, y) = (1, 2, 3); // expected a tuple with 3 elements, found one with 2 elements
        */
    }
}
