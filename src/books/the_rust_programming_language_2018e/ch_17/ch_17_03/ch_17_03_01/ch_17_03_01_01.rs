/*
  Object Oriented Programming Features of Rust
      Implementing an Object-Oriented Design Pattern
          Defining Post and Creating a New Instance in the Draft State
*/

pub fn fn_17_03_01_01() {
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
        }

        trait State {}

        struct Draft {}

        impl State for Draft {}
    }
}
