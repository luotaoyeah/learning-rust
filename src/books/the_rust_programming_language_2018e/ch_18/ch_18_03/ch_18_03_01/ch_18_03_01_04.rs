/*
  Patterns and Matching
      Pattern Syntax
          Matching Ranges of Values with ...
*/

pub fn fn_18_03_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          可以使用  x...y 的形式，表示匹配某个范围，
          只能用于数字（numeric）或者字符（char）；
        */

        let x = 5;
        match x {
            1...5 => println!("1...5"),
            _ => println!("other"),
        }

        let c = 'b';
        match c {
            'a'...'z' => println!("a-z"),
            _ => println!("other"),
        }
    }
}
