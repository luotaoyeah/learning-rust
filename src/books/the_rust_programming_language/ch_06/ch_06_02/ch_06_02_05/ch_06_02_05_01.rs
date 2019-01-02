/*
   Enums and Pattern Matching
       match
           the `_` placeholder
 */

pub fn fn_06_02_05_01() {
    println!("-------------------------------------------------- 01");
    {
        /*
           因为 match 是 exhaustive 的，必须包含所有可能的情况，
           可以使用 _ 匹配所有没有显式声明的情况，
           类似其他语言 switch 语句的 default 分支；
         */

        /// u8 类型的 x 可能的值为 0 ~ 255，可以显示声明 1，3，5，
        /// 然后使用 _ 来匹配其他所有的值；
        fn some_u8_values(x: u8) {
            match x {
                1 => println!("one"),
                3 => println!("three"),
                5 => println!("five"),
                _ => (),
            }
        }

        some_u8_values(3); // three
        some_u8_values(9);
    }
}
