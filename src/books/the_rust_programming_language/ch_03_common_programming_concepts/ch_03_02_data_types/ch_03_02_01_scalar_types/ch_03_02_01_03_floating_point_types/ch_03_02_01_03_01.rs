/*
  Common Programming Concepts
      Data Types
          Scalar Types
              Floating-Point Types
*/

pub fn fn_03_02_01_03_01() {
    println!("-------------------------------------------------- 01");
    /*
      rust 有两个表示浮点数的类型：f32 和 f64
    */

    /* 默认的浮点数类型是 f64 */
    let f01 = 3.14;
    crate::print_type_of(&f01); // f64

    let f02: f32 = 3.14;
    crate::print_type_of(&f02); // f32
}
