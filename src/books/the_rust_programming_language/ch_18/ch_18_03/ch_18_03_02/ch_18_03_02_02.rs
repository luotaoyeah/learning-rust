/*
  Patterns and Matching
      Pattern Syntax
          Destructuring to Break Apart Values
              Destructuring Enums
*/

pub fn fn_18_03_02_02() {
    println!("-------------------------------------------------- 01");
    {
        match_msg(Message::Quit); // QUIT
        match_msg(Message::Move { x: 9, y: 9 }); // 9 9
        match_msg(Message::Write(String::from("foo"))); // foo
        match_msg(Message::ChangeColor(1, 2, 3)); // 1 2 3
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn match_msg(msg: Message) {
    match msg {
        Message::Quit => println!("QUIT"),
        Message::Move { x, y } => println!("{} {}", x, y),
        Message::Write(s) => println!("{}", s),
        Message::ChangeColor(r, g, b) => println!("{} {} {}", r, g, b),
    }
}
