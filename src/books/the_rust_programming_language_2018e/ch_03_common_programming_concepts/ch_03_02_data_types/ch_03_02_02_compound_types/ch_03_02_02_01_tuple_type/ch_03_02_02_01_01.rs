/*
  Common Programming Concepts
      Data Types
          Compound Types
              The Tuple Type
*/

pub fn fn_03_02_02_01_01() {
    println!("-------------------------------------------------- 01");
    /*
      compound type 表示组合类型，rust 有两个原始的组合类型：tuple 和 array
      tuple 表示元组类型，是将多个不同的类型组合在一起，形成一个新的组合类型，
      tuple 的语法如下：
    */
    let tuple01: (i32, f64, u8) = (99, 9.99, 9);
    println!("{:?}", tuple01); // (99, 9.99, 9)
}
