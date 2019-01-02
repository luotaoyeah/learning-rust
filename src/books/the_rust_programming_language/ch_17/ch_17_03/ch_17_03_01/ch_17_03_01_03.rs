/*
  Object Oriented Programming Features of Rust
      Implementing an Object-Oriented Design Pattern
          Ensuring the Content of a Draft Post Is Empty
*/

pub fn fn_17_03_01_03() {
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

            pub fn content(&self) -> &str {
                ""
            }
        }

        trait State {}

        struct Draft {}

        impl State for Draft {}
    }
}
