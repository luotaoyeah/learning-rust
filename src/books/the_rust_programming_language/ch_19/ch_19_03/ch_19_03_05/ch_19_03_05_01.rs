/*
  Advanced Features
      Advanced Traits
          Using the Newtype Pattern to Implement External Traits on External Types
*/

use std::fmt;

pub fn fn_19_03_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          rust 有一个 orphan rule：
              当某个类型实现某个 trait 时，类型或者 trait 必须至少有一个是属于当前 crate 的，
              即不能为外部的类型实现外部的 trait；
        
          可以使用 newtype 模式简介实现这个功能，newtype 是 haskell 中的一个术语；
        */

        /*
          使用一个 tuple struct 对外部的类型进行包装
        */
        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "[ {} ]", self.0.join(", "))
            }
        }

        println!(
            "{}",
            Wrapper(vec![String::from("foo"), String::from("bar")])
        ); // [ foo, bar ]
    }
}
