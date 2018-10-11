/*
   Common Programming Concepts
       Data Types
           Scalar Types
               Integer Types
                   Integer Overflow
 */

/*
   每一个整数类型所能表示的数字范围都是确定的，如 u8 表示的范围是：0-255，
   如果将超出范围的值赋值给变量，会发生整数溢出（integer overflow），
   如将 257 赋值给 u8 类型的变量；
   TODO 待确认
       在发布模式下（cargo build --release），rust 会执行 two's compliment wrapping，
       将 257 变为 1，将 258 变为 2，依次类推；
 */

pub fn fn_03_02_03() {
    let x: u8 = 257; // warning: literal out of range for u8

    println!("{}", x); // 1
}
