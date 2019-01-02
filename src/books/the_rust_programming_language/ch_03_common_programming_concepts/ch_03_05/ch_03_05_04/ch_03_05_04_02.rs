/*
   Control Flow
       Repetition with Loops
           Returning from loops
 */

pub fn fn_03_05_04_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           如果 break 后面跟上一个 expression，
           则该 expression 会返回成为 loop 的值；
         */

        let mut n: u32 = 1;

        let m: u32 = loop {
            if n >= 100 {
                break n / 2;
            }

            n += 1;
        };

        println!("m: {}", m); // m: 50
    }
}
