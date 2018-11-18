/*
  Advanced Features
      Advanced Traits
          Using Supertraits to Require One Trait’s Functionality Within Another Trait
*/

use std::fmt;

pub fn fn_19_03_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          如果某个 trait A 依赖于其他的 trait B，
          即某个类型要实现 A，则必须也实现 B，
          则可以将 B 声明为 A 的 supertrait；
        */

        /*
          Display 是 Print 的 supertrait
        */
        trait Print: fmt::Display {
            fn print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        impl Print for Point {}

        let point = Point { x: 1, y: 1 };
        point.print();
    }
}
