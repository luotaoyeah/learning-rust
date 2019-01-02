/*
  Patterns and Matching
      Pattern Syntax
          Destructuring to Break Apart Values
              Destructuring Nested Structs & Enums
*/

pub fn fn_18_03_02_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
          嵌套的 enum 和 struct 也可以使用 pattern destructure；
        */

        match_msg(Message::Quit); // QUIT
        match_msg(Message::ChangeColor(Color::Rgb(1, 2, 3))); // rgb: 1 2 3
        match_msg(Message::ChangeColor(Color::Hsv(4, 5, 6))); // hsv: 4 5 6
    }
}

enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

enum Message {
    Quit,
    ChangeColor(Color),
}

fn match_msg(msg: Message) {
    match msg {
        Message::Quit => println!("QUIT"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb: {} {} {}", r, g, b),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("hsv: {} {} {}", h, s, v),
    }
}
