/*
   Control Flow
       Repetition with Loops
           Repeating Code with loop
 */

pub fn fn_03_05_04_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           rust 提供了3种循环方式：loop，while，for；
         */

        /*
           loop 用来循环执行一段代码；
           可以通过 break 终止循环；
         */

        let mut n = 0;

        loop {
            if n >= 100 {
                break;
            }

            n += 1;
            println!("{}", n);
        }
    }
}
