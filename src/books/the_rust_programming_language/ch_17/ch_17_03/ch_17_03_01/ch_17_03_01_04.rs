/*
  Object Oriented Programming Features of Rust
      Implementing an Object-Oriented Design Pattern
          Requesting a Review of the Post Changes Its State
*/

pub fn fn_17_03_01_04() {
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

            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = s.request_review();
                }
            }
        }

        trait State {
            /*
              方法的第一个参数 self 定义为 Box<Self> 类型，
              表示调用方法的对象必须是一个 Box<T> 类型对象，
              切回获取调用对象的 ownership；
            */
            fn request_review(self: Box<Self>) -> Box<dyn State>;
        }

        struct Draft {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }
        }

        struct PendingReview {}

        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
    }
}
