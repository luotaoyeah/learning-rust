/*
  Common Programming Concepts
      Data Types
          Scalar Types
              Floating-Point Types
*/

/*
  rust 中浮点数有两种类型，分别为：f32（单精度浮点数）和 f64（双精度浮点数），遵循 IEEE-754 标准；
  默认浮点数类型为 f64；
*/
pub fn fn_03_02_04() {
    // 默认为 f64
    let x = 3.14; // f64

    // 显式指定为 f32
    let y: f32 = 1.23; // f32

    println!("{}", x / y); // 2.5528455
}
