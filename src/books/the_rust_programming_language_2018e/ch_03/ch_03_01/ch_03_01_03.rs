/*
  Common Programming Concepts
      Shadowing
*/

pub fn fn_03_01_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          使用 let 重复声明同名的变量，后面的变量会隐藏之前声明的变量，称之为 shadowing；
        */
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("{}", x); // 12
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          shadowing 的时候，可以使用不同的类型；
        */
        let spaces = "foo";
        let mut spaces = spaces.len();
        spaces += 1;
        println!("{}", spaces); // 4
    }
}
