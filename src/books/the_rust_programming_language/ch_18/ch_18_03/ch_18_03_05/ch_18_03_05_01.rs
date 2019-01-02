/*
  Patterns and Matching
      Pattern Syntax
          @ Bindings
*/

pub fn fn_18_03_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          @ 用在 pattern 中，在匹配的同时，将匹配的值绑定到变量；
        */

        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id @ 3...9 } => println!("{}", id),
            Message::Hello { id: 99 } => println!("99"),
            _ => (),
        }
    }
}
