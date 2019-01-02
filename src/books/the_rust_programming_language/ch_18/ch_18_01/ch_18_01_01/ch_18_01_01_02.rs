/*
  Patterns and Matching
      All the Places Patterns Can Be Used
          Conditional if let Expressions
*/

pub fn fn_18_01_01_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           当只对 match 表达式的某一个 arm 感兴趣时，
           可以使用 if let 语法糖；
           if let 可以跟 else if 和 else if let 混用；
        */

        let favorite_color: Option<&str> = None;
        let is_tuesday: bool = false;
        let age: Result<u8, _> = "34".parse::<u8>();

        if let Some(c) = favorite_color {
            println!("{}", c);
        } else if is_tuesday {
            println!("green");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("yellow");
            } else {
                println!("red");
            }
        } else {
            println!("black");
        }
    }

    println!("-------------------------------------------------- 02");
    {
        let o: Option<i32> = Some(9);
        let i = 0;

        if i > 0 {
            println!("i > 0");
        } else if let Some(x) = o {
            println!("x: {}", x);
        } else {
            println!("no");
        }
    }
}
