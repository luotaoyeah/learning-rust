/*
  Object Oriented Programming Features of Rust
      Using Trait Objects that Allow for Values of Different Types
          Implementing the Trait
*/

pub fn fn_17_02_01_02() {
    println!("-------------------------------------------------- 01");
    {
        pub trait Draw {
            fn draw(&self);
        }

        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }

        impl Screen {
            fn run(&self) {
                for c in self.components.iter() {
                    c.draw();
                }
            }
        }

        pub struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }

        impl Draw for Button {
            fn draw(&self) {
                println!("Button.draw()");
            }
        }

        pub struct Select {
            pub width: u32,
            pub height: u32,
            options: Vec<String>,
        }

        impl Draw for Select {
            fn draw(&self) {
                println!("Select.draw()");
            }
        }

        let screen = Screen {
            components: vec![
                Box::new(Button {
                    height: 9,
                    width: 9,
                    label: String::from("button"),
                }),
                Box::new(Select {
                    width: 18,
                    height: 9,
                    options: vec![
                        String::from("A"),
                        String::from("B"),
                        String::from("C"),
                        String::from("D"),
                    ],
                }),
            ],
        };

        screen.run();
    }
}
