/*
   Functions
       Functions with Return Values
 */

pub fn fn_03_03_05() {
    println!("-------------------------------------------------- 01");
    {
        fn plus_one(i: i32) -> i32 {
            /*
               使用 return statement 显式返回；
             */
            return i + 1;
        }

        fn minus_one(i: i32) -> i32 {
            /*
               使用 statement 隐式返回；
             */
            i - 1
        }

        println!("{}", plus_one(5));
        println!("{}", minus_one(5));
    }
}
