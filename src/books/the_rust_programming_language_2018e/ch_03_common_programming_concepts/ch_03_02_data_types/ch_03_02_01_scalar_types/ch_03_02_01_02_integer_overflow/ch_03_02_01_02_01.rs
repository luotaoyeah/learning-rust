/*
  Common Programming Concepts
      Data Types
          Scalar Types
              Integer Types
                  Integer Overflow
*/

/*
  每一个整数类型所能表示的范围都是确定的，如 u8 表示的范围是：0-255
  如果将超出范围的值赋值给变量，会发生整数溢出（integer overflow）
  比如：将 256 赋值为 u8 类型的变量

  因为 #![deny(overflowing_literals)] 是默认开启的，因此会编译报错
  如果将 #![allow(overflowing_literals)] 开启，则在编译时，
  会自动使用 two's complement wrapping 进行转换，
  如，将 256 转换为 0，将 257 转换为 1，以此类推
*/

#![allow(overflowing_literals)]
pub fn fn_03_02_01_02_01() {
    let x: u8 = 257; // error: literal out of range for u8
    println!("{}", x); // 1
}
