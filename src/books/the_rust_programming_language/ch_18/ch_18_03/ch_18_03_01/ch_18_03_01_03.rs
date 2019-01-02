/*
  Patterns and Matching
      Pattern Syntax
          Multiple Patterns
*/

pub fn fn_18_03_01_03() {
    println!("-------------------------------------------------- 01");
    {
        let x = 2;

        /*
          使用 | 连接多个 pattern，表示只要其中某个 pattern 匹配，
          整个 pattern 就算匹配；
        */
        match x {
            1 | 2 => {
                println!("1 or 2");
            }
            _ => println!("other"),
        }
    }
}
