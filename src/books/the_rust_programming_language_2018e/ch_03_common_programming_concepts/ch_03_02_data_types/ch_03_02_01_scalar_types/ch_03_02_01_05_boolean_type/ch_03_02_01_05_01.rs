/*
  Common Programming Concepts
      Data Types
          Scalar Types
              The Boolean Type
*/

use std::mem;

pub fn fn_03_02_01_05_01() {
    println!("-------------------------------------------------- 01");
    /*
       布尔类型使用 bool 标识
    */
    let mut b: bool = true;
    println!("{}", b); // true
    b = false;
    println!("{}", b); // false

    println!("-------------------------------------------------- 02");
    /*
      bool 类型占用一个字节（byte）
    */
    println!("{}", mem::size_of::<bool>()); // 1
}
