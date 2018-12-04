/*
   Control Flow
       if expressions
           handling multiple conditions with else if
 */

pub fn fn_03_05_02() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 if else/if else 实现多个条件分支；
           如果分支过多，应该使用 match 语句；
         */

        let n: i32 = 6;

        if n % 4 == 0 {
            println!("4");
        } else if n % 3 == 0 {
            println!("3");
        } else if n % 2 == 0 {
            println!("2");
        } else {
            println!("-1");
        }
    }
}
