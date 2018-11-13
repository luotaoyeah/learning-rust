/*
  Object Oriented Programming Features of Rust
      Implementing an Object-Oriented Design Pattern
          Storing the Text of the Post Content
*/

pub fn fn_17_03_01_02() {
    println!("-------------------------------------------------- 01");
    {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }

            pub fn add_text(&mut self, txt: &str) {
                self.content.push_str(txt);
            }
        }

        trait State {}

        struct Draft {}

        impl State for Draft {}
    }
}
