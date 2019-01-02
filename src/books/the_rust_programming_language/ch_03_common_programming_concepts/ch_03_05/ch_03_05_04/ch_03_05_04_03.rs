/*
   Control Flow
       Repetition with Loops
           Conditional Loops with while
 */

pub fn fn_03_05_04_03() {
    println!("-------------------------------------------------- 01");
    {
        /*
           while 循环会在每次循坏之前判断 expression 是否为 true，
           如果为 true，则继续下一次循环，
           如果为 false，则终止循环；
         */

        let mut x = 1;
        while x <= 5 {
            println!("x: {}", x);
            x += 1;
        }
    }

    println!("-------------------------------------------------- 02");
    {
        /*
           可以使用 loop 模拟 while 循环；
         */

        let mut y = 1;
        loop {
            if y > 5 {
                break;
            }

            println!("y: {}", y);
            y += 1;
        }
    }
}
