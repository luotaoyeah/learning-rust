/*
  Common Programming Concepts
      Data Types
          Compound Types
              The Array Type
                  Invalid Array Element Access
*/

#![deny(const_err)]
pub fn fn_03_02_02_02_04() {
    println!("-------------------------------------------------- 01");
    /*
       当访问数组元素的索引超出范围时，
       如果开启了 #![allow(const_err)]，则编译通过，运行报错，
       如果开启了 #![deny(const_err)]，则编译就会报错
    */

    let arr01: [&str; 3] = ["a", "b", "c"];
    let x: &str = arr01[9]; // error: index out of bounds: the len is 3 but the index is 9
    println!("{}", x);
}
