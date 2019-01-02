/*
  Patterns and Matching
      Pattern Syntax
          Extra Conditionals with Match Guards
*/

pub fn fn_18_03_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          match guard 是 match 的 arm 的 pattern 后面的一个 if 语句，
          此时，当该 arm 的 pattern 匹配，并且后面的 if 语句也成立时，
          整个 arm 才会匹配；
        */

        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("{} < 5", x), // 4 < 5
            Some(x) => println!("{}", x),
            _ => (),
        }
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          当 arm 是一个 | 连接的多个 pattern 时，
          后面的 match guard 会作用于每一个 pattern；
        */

        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("{}", x),
            _ => println!("other"), // other
        }
    }
}
