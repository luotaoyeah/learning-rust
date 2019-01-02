/*
   Control Flow
       Repetition with Loops
           Looping Through a Collection with for
 */

pub fn fn_03_05_04_05() {
    println!("-------------------------------------------------- 01");
    {
        /*
           for 结合 range 可以模拟 while；
         */

        for n in (1..4).rev() {
            println!("{}", n);
        }
    }
}
