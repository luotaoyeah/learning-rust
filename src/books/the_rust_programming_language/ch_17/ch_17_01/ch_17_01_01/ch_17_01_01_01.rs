/*
  Object Oriented Programming Features of Rust
      Characteristics of Object-Oriented Languages
          Objects Contain Data and Behavior
          Encapsulation that Hides Implementation Details
*/

pub fn fn_17_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          在 rust 中，struct 和 enum 提供了 data，
          对应的 impl block 提供了 method，因此可以看成是 OOP，
          且 rust 提供了 pub 关键字，用于暴露公共接口；
        */

        pub struct AverageCollection {
            list: Vec<i32>,
            avg: f64,
        }

        impl AverageCollection {
            /// update average
            fn update_avg(&mut self) {
                let sum: i32 = self.list.iter().sum();
                self.avg = sum as f64 / self.list.len() as f64;
            }

            /// get average
            pub fn avg(&self) -> f64 {
                self.avg
            }

            /// add an element
            pub fn add(&mut self, x: i32) {
                self.list.push(x);
                self.update_avg();
            }

            /// remove an element
            pub fn remove(&mut self) -> Option<i32> {
                let el = self.list.pop();
                match el {
                    Some(x) => {
                        self.update_avg();
                        Some(x)
                    }
                    None => None,
                }
            }
        }
    }
}
