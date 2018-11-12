/*
  Object Oriented Programming Features of Rust
      Using Trait Objects that Allow for Values of Different Types
          Defining a Trait for Common Behavior
*/

pub fn fn_17_02_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          trait object：
              某个实现指定特性（trait）的类型（type）的一个实例的引用（reference）
          写法：
              &dyn Trait 或者 Box<dyn Trait>
          作用：
              抽象某个通用行为
        */

        pub trait Draw {
            fn draw(&self);
        }

        pub struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }

        impl Screen {
            pub fn run(&self) {
                for c in self.components.iter() {
                    c.draw();
                }
            }
        }
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          如果使用 generic type + trait bounds，
          则 generic type parameter 只能被替换为某个实际类型，
          即 components 中要么全都是类型 A，要么全都是类型 B；
          而使用 trait objects，则 components 可以同时包含类型 A 和类型 B，
          只要 A 和 B 都实现了指定的 trait；
        */
        pub trait Draw {
            fn draw(&self);
        }

        pub struct Screen<T>
        where
            T: Draw,
        {
            pub components: Vec<T>,
        }

        impl<T> Screen<T>
        where
            T: Draw,
        {
            fn run(&self) {
                for c in self.components.iter() {
                    c.draw();
                }
            }
        }
    }
}
