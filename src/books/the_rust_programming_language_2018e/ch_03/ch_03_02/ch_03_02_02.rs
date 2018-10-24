/*
   Common Programming Concepts
       Data Types
           Scalar Types
               Integer Types
 */

/*
   scalar type 表示单个值，rust 有四种 scalar types：
       integer（整数）
       floating point number（浮点数）
       boolean（布尔值）
       character（字符）
 */

/*
   integer 表示一个整数（没有小数部分），分为'有符号整数'和'无符号整数'，
   '有符号整数'以 i 开头，'无符号整数'以 u 开头：
       i8       u8
       i16      u16
       i32      u32
       i64      u64
       i128     u128
       isize    usize

   '有符号整数'表示的范围为：-2^(n-1) 到 2^(n-1)-1
       如：i8 表示 -128 到 127
   '无符号整数'表示的范围为：0 到 2^n-1
       如：u8 表示 0 到 255

   isize 和 usize 根据程序运行的系统平台，来决定位数；64位操作系统上为64位（i64/u64），32位操作系统上为32位（i32/u32）；

   默认的整数类型是 i32；
 */

pub fn fn_03_02_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           整数字面量（integer literal）有五种表示形式
         */
        let a = 01; // 十进制
        let b = 0b01; // 二进制
        let c = 0o01; // 八进制
        let d = 0x01; // 十六进制
        let e: u8 = b'A'; // 字节

        println!("{}, {}, {}, {}, {}", a, b, c, d, e); // 1, 1, 1, 1, 65
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           除了字节以外，其他四种类型的整数字面量，可以加上后缀来表示数据类型；
         */
        let a = 01u8;
        let b = 0b01u8;
        let c = 0o01u8;
        let d = 0x01u8;
        let e = b'A';

        println!("{}, {}, {}, {}, {}", a, b, c, d, e); // 1, 1, 1, 1, 65
    }

    println!("-------------------------------------------------- 03");
    {
        /*
           在 integer literal 中可以使用下划线（_）来分隔数字；
         */
        let a = 1000_000;
        let b = 0b0101_0101;
        println!("{}, {}", a, b); // 1000000, 85
    }
}
