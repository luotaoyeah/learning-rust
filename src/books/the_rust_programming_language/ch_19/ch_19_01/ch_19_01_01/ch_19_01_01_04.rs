/*
  Advanced Features
      Unsafe Rust
          Using extern Functions to Call External Code
*/

use std::slice;

pub fn fn_19_01_01_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
          extern 关键字用来定义一个或者使用一个 FFI（Foreign Function Interface），
          用来跟其他语言进行交互；
        
          extern "C" 中的 "C" 表示该 FFI 用的是哪一种 ABI（Application Binary Interface）；
        */
        extern "C" {
            fn abs(n: i32) -> i32;
        }

        unsafe {
            println!("{}", abs(-3)); // 3
        }
    }
}
