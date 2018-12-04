/*
   Control Flow
       if expressions
           using if in a let statement
 */

extern crate rand;

use self::rand::Rng;

pub fn fn_03_05_03_02() {
    println!("-------------------------------------------------- 01");
    {
        let n = rand::thread_rng().gen_range(1, 101);

        /*
           当 if 表达式用在 let 语句的右边时，因为 if 表达式最终的值有可能是任意一个分支的值，
           所以 if 表达式所有分支的值的类型必须相同，否则报错；
           因为 rust 必须在编译时期确切地知道变量的类型，否则无法进行类型检查；
         */
        let m = if n % 2 == 0 { "EVEN" } else { false }; // [E0308]: if and else have incompatible types
        println!("{}", m);
    }
}
