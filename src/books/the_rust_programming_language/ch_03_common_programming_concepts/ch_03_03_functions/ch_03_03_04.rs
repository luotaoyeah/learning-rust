/*
   Functions
       Functions with Return Values
 */

pub fn fn_03_03_04() {
    println!("-------------------------------------------------- 01");
    {
        /*
           使用 -> 来定义一个函数的返回类型；
         */
        fn fn_01() -> i32 {
            /*
               函数体中，最后一个 expression 默认就是函数的返回值；
             */
            5
        }

        let x = fn_01();
        println!("x: {}", x);
    }

    println!("-------------------------------------------------- 02");
    {
        fn fn_02(x: i32) -> char {
            if x > 0 {
                /*
                   可以使用 return 关键字，显式返回；
                 */
                return '>';
            };

            '<'
        }

        println!("{} {}", fn_02(-1), fn_02(1)); // < >
    }
}
