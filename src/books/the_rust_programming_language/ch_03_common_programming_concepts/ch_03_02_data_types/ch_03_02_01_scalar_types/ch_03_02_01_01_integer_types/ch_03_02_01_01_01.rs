/*
  Common Programming Concepts
      Data Types
          Scalar Types
              Integer Types
*/

/*
  scalar type 表示单一类型，rust 有四种 scalar types：
      integer（整数）
      floating point number（浮点数）
      boolean（布尔值）
      character（字符）
*/

/*
  integer 表示整数（没有小数部分），分为'有符号整数'和'无符号整数'，
  '有符号整数'以 i 开头，'无符号整数'以 u 开头：
      i8               u8
      i16              u16
      i32              u32
      i64              u64
      i128             u128
      isize            usize

  默认的整数类型是 i32
*/

/*
  '有符号整数'表示的范围为：-2^(n-1) 到 2^(n-1)-1
      如：i8 表示 -128 到 127
  '无符号整数'表示的范围为：0 到 2^n-1
      如：u8 表示 0 到 255
*/

/*
  isize 和 usize 根据程序运行的系统平台来决定位数，
  64位操作系统上为64位（i64/u64），
  32位操作系统上为32位（i32/u32）
*/
pub fn fn_03_02_01_01_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
          integer literal 有五种形式：
        */

        /* 十进制（decimal） */
        let a: u32 = 9;

        /* 十六进制（hexadecimal） */
        let b: u32 = 0xf;

        /* 八进制（octal） */
        let c: u32 = 0o7;

        /* 二进制（binary） */
        let d: u32 = 0b1;

        /* btype literal */
        /*
          类型为 u8 的整数，可以使用 byte literal 的形式来表示，
          即，使用 b'C' 的形式来表示，其中 C 表示某个字符
        */
        let e: u8 = b'A';

        println!("{}, {}, {}, {}, {}", a, b, c, d, e); // 9, 15, 7, 1, 65
    }
}
