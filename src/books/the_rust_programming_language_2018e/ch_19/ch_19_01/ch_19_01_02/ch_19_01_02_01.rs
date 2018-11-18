/*
  Advanced Features
      Unsafe Rust
          Accessing or Modifying a Mutable Static Variable
*/

use std::slice;

pub fn fn_19_01_02_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 中的全局变量称之为 static variable，使用关键字 static 定义；
           包括 immutable static variable 和 mutable static variable；
           static variable 必须显式声明类型；
        */

        static NUM: i32 = 9;
        static STR: &str = "foo";

        println!("{}", NUM); // 9
        println!("{}", STR); // foo
    }

    println!("-------------------------------------------------- 02");
    {
        /*
          mutable static variable 是 unsafe 的；
        */

        static mut COUNTER: i32 = 1;

        unsafe {
            COUNTER += 1;
        }

        unsafe {
            println!("{}", COUNTER); // 2
        }
    }
}
