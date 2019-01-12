/*
   Control Flow
       if expressions
           using if in a let statement
 */

extern crate rand;

use self::rand::Rng;

pub fn fn_03_05_03_01() {
    println!("-------------------------------------------------- 01");
    {
        let n = rand::thread_rng().gen_range(1, 101);

        /*
           因为 if 是一个 expression，因此可以用在 let 赋值语句的 = 右边，
           因为代码块是一个 expression，它的值为代码块中最后一个 expression 的值，
           而 if 后面和 else 后面都分别是一个代码块，所以 if expression 最终的值为某个分支的值；
         */
        let m: &str = if n % 2 == 0 { "EVEN" } else { "ODD" };
        println!("{}", m);
    }
}
